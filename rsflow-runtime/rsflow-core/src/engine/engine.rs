use crate::core::{
    EngineMessage, EngineSender, FlowContext, Node, NodeBuilder, NodeInput, NodeOutput,
    NodeRunItem, Payload,
};
use crate::engine::flow_processor::FlowProcessor;
use crate::flow::FlowMod;

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

use tokio::sync::{Mutex, mpsc};
use uuid::Uuid;

type NodeMap = HashMap<Uuid, Arc<dyn Node + Send + Sync>>;
type Nodes = Arc<NodeMap>;
type BuilderMap = HashMap<String, Box<dyn NodeBuilder>>;

pub struct Engine {
    flow_mod: FlowMod,
    nodes: Nodes,
    receiver: Mutex<mpsc::Receiver<EngineMessage>>,
    sender: mpsc::Sender<EngineMessage>,
}

impl Engine {
    pub fn get_mod(&self) -> &FlowMod {
        &self.flow_mod
    }

    /// ⚠️ 只能通过 Builder 调用
    pub async fn create_with_builders(
        flow_file_path: &str,
        builders: BuilderMap,
    ) -> Result<Arc<Self>, std::io::Error> {
        // 解析流程文件并验证
        let flow_mod = FlowProcessor::parse_flow_file(flow_file_path)?;

        // 提取节点和节点类型
        let rsflow_nodes = FlowProcessor::extract_nodes(&flow_mod);
        let node_types = FlowProcessor::extract_node_types(&rsflow_nodes);

        // 将构建器转换为工厂
        let factories = FlowProcessor::builders_to_factories(
            builders,
            &node_types,
            &flow_mod.node_global_config,
        )
        .await?;

        // 创建节点实例
        let nodes = FlowProcessor::create_nodes_from_flow(
            rsflow_nodes,
            &factories,
            &flow_mod.node_global_config,
        )
        .await?;

        let (tx, rx) = mpsc::channel(flow_mod.config.msg_len);

        Ok(Arc::new(Self {
            flow_mod,
            nodes: Arc::new(nodes),
            receiver: Mutex::new(rx),
            sender: tx,
        }))
    }

    /// 启动 Engine（生命周期锚点）
    pub async fn start(self: Arc<Self>) {
        println!("Starting engine...");

        // 初始化节点
        for (node_id, node) in self.nodes.iter() {
            let engine_ctx = EngineSender {
                tx: self.sender.clone(),
            };
            println!("Initializing node: {} - {}", node_id, node.info().name);
            node.init(engine_ctx).await;
        }

        println!("Engine started. Waiting for messages...");

        // 消息循环
        let mut rx = self.receiver.lock().await;
        while let Some(msg) = rx.recv().await {
            match msg {
                EngineMessage::RunFlow { ctx, start_node } => {
                    self.flow_run(ctx, start_node);
                }
                EngineMessage::NodeEvent {
                    node_id,
                    ctx,
                    event_type,
                    payload,
                } => {
                    self.node_event(node_id, event_type, ctx, payload);
                }
                EngineMessage::Stop => {
                    println!("Engine stopping...");
                    break;
                }
            }
        }

        println!("Engine stopped.");
    }

    /// 节点消息事件
    fn node_event(&self, node_id: Uuid, event_type: String, ctx: FlowContext, payload: Payload) {
        let nodes: Nodes = Arc::clone(&self.nodes);

        tokio::spawn(async move {
            if let Some(node) = nodes.get(&node_id) {
                if let Err(err) = node.event(&event_type, payload, &ctx).await {
                    eprintln!("Node event error: {:?}", err);
                }
            }
        });
    }

    /// 核心调度逻辑
    fn flow_run(&self, mut ctx: FlowContext, start_node: NodeRunItem) {
        let nodes: Nodes = Arc::clone(&self.nodes);
        let sender = self.sender.clone();

        tokio::spawn(async move {
            let mut flow_run_node_ids: VecDeque<NodeRunItem> = VecDeque::new();
            flow_run_node_ids.push_back(start_node);

            while let Some(node_run_item) = flow_run_node_ids.pop_front() {
                let Some(node) = nodes.get(&node_run_item.node_id) else {
                    eprintln!("Node {} not found", node_run_item.node_id);
                    continue;
                };

                match node.input(node_run_item.node_input, &ctx).await {
                    Ok(node_output) => {
                        ctx.run_node_ids.push(node_run_item.node_id);
                        match node_output {
                            NodeOutput::None => continue,
                            NodeOutput::One((port, msg)) => {
                                // 获取执行node的输出节点定义
                                let out_ids = node.info().output_ports;
                                if let Some(out_node_ids) = out_ids.get(&port) {
                                    if out_node_ids.len() == 1 {
                                        // 单分支，继续当前线程执行
                                        let (out_node_id, out_node_port) = &out_node_ids[0];
                                        flow_run_node_ids.push_back(NodeRunItem {
                                            node_id: *out_node_id,
                                            node_input: NodeInput {
                                                port: *out_node_port,
                                                msg: msg,
                                            },
                                        });
                                    } else {
                                        // 多分支，第一个分支继续当前线程执行，其余分支发送RunFlow消息
                                        let mut iter = out_node_ids.iter();
                                        if let Some((first_node_id, first_node_port)) = iter.next()
                                        {
                                            // 第一个分支继续当前线程执行
                                            flow_run_node_ids.push_back(NodeRunItem {
                                                node_id: *first_node_id,
                                                node_input: NodeInput {
                                                    port: *first_node_port,
                                                    msg: msg.clone(),
                                                },
                                            });
                                        }

                                        // 其余分支发送RunFlow消息
                                        for (out_node_id, out_node_port) in iter {
                                            let branch_ctx = ctx.new_branch();

                                            let branch_node_run_item = NodeRunItem {
                                                node_id: *out_node_id,
                                                node_input: NodeInput {
                                                    port: *out_node_port,
                                                    msg: msg.clone(),
                                                },
                                            };

                                            if let Err(e) = sender
                                                .send(EngineMessage::RunFlow {
                                                    ctx: branch_ctx,
                                                    start_node: branch_node_run_item,
                                                })
                                                .await
                                            {
                                                eprintln!(
                                                    "Failed to send RunFlow message: {:?}",
                                                    e
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                            NodeOutput::Many(msgs) => {
                                // 获取执行node的输出节点定义
                                let out_ids = node.info().output_ports;
                                for (_, (port, msg)) in msgs.iter().enumerate() {
                                    if let Some(out_node_ids) = out_ids.get(port) {
                                        if out_node_ids.len() == 1 {
                                            // 单分支，继续当前线程执行
                                            let (out_node_id, out_node_port) = &out_node_ids[0];
                                            flow_run_node_ids.push_back(NodeRunItem {
                                                node_id: *out_node_id,
                                                node_input: NodeInput {
                                                    port: *out_node_port,
                                                    msg: msg.clone(),
                                                },
                                            });
                                        } else {
                                            // 多分支，第一个分支继续当前线程执行，其余分支发送RunFlow消息
                                            let mut iter = out_node_ids.iter();
                                            if let Some((first_node_id, first_node_port)) =
                                                iter.next()
                                            {
                                                flow_run_node_ids.push_back(NodeRunItem {
                                                    node_id: *first_node_id,
                                                    node_input: NodeInput {
                                                        port: *first_node_port,
                                                        msg: msg.clone(),
                                                    },
                                                });
                                            }

                                            // 其余分支发送RunFlow消息
                                            for (out_node_id, out_node_port) in iter {
                                                let branch_ctx = ctx.new_branch();

                                                let branch_node_run_item = NodeRunItem {
                                                    node_id: *out_node_id,
                                                    node_input: NodeInput {
                                                        port: *out_node_port,
                                                        msg: msg.clone(),
                                                    },
                                                };

                                                if let Err(e) = sender
                                                    .send(EngineMessage::RunFlow {
                                                        ctx: branch_ctx,
                                                        start_node: branch_node_run_item,
                                                    })
                                                    .await
                                                {
                                                    eprintln!(
                                                        "Failed to send RunFlow message: {:?}",
                                                        e
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!(
                            "Node {} - {} error: {:?}",
                            node_run_item.node_id,
                            node.info().name,
                            err
                        );
                        continue;
                    }
                }
            }
        });
    }
}
