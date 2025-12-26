use crate::core::{
    EngineMessage, EngineSender, FlowContext, Node, NodeBuilder, NodeFactory, NodeInfo, NodeOutput,
    NodePorts, NodeRunItem, Value,
};
use crate::flow::{
    FlowMod, parse_flow_all_node_types, parse_flow_all_nodes, parse_flow_file, validate_flow,
};

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

use tokio::sync::{Mutex, mpsc};
use uuid::Uuid;

type FactoryMap = HashMap<String, Box<dyn NodeFactory>>;
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
    /// ⚠️ 只能通过 Builder 调用
    pub async fn create_with_builders(
        flow_file_path: &str,
        builders: BuilderMap,
    ) -> Result<Arc<Self>, std::io::Error> {
        use std::io::ErrorKind;

        // 解析 flow
        let flow_mod = parse_flow_file(flow_file_path)?;

        // 验证 flow 配置
        if let Err(err) = validate_flow(&flow_mod) {
            return Err(std::io::Error::new(
                ErrorKind::InvalidData,
                format!("Flow validation failed: {}", err),
            ));
        }

        let flow_nodes = parse_flow_all_nodes(flow_mod.clone());
        let node_types = parse_flow_all_node_types(flow_nodes.clone());

        // builder -> factory
        let mut factories: FactoryMap = HashMap::new();
        let mut missing_builders = Vec::new();

        for t in &node_types {
            if let Some(builder) = builders.get(t) {
                match builder.register().await {
                    Ok(factory) => {
                        factories.insert(t.clone(), factory);
                    }
                    Err(err) => {
                        return Err(std::io::Error::new(
                            ErrorKind::Other,
                            format!("Failed to register factory for node type {}: {:?}", t, err),
                        ));
                    }
                }
            } else {
                missing_builders.push(t.clone());
            }
        }

        if !missing_builders.is_empty() {
            return Err(std::io::Error::new(
                ErrorKind::InvalidData,
                format!("Missing builders for node types: {:?}", missing_builders),
            ));
        }

        // factory -> node
        let mut nodes: NodeMap = HashMap::new();
        for flow_node in &flow_nodes {
            if let Some(factory) = factories.get(&flow_node.node_type) {

                // 将 FlowNodeInput 转换为 NodePorts
                let inputs: NodePorts = flow_node
                    .input
                    .iter()
                    .map(|in_| (in_.port, in_.nodes.clone()))
                    .collect();

                // 将 FlowNode 转换为 NodeInfo
                let outputs: NodePorts = flow_node
                    .output
                    .iter()
                    .map(|out| (out.port, out.nodes.clone()))
                    .collect();

                let node_info = NodeInfo {
                    id: flow_node.id,
                    name: flow_node.name.clone(),
                    node_type: flow_node.node_type.clone(),
                    description: flow_node.description.clone(),
                    config: flow_node.config.clone(),
                    input_ports: inputs,
                    output_ports: outputs,
                };

                match factory.create(node_info).await {
                    Ok(node) => {
                        nodes.insert(flow_node.id, node);
                    }
                    Err(err) => {
                        return Err(std::io::Error::new(
                            ErrorKind::Other,
                            format!("Failed to create node {}: {:?}", flow_node.id, err),
                        ));
                    }
                }
            }
        }

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
            let sender = EngineSender {
                tx: self.sender.clone(),
            };
            println!("Initializing node: {} - {}", node_id, node.info().name);
            node.on_start(sender).await;
        }

        println!("Engine started. Waiting for messages...");

        // 消息循环
        let mut rx = self.receiver.lock().await;
        while let Some(msg) = rx.recv().await {
            match msg {
                EngineMessage::RunFlow {
                    ctx,
                    start_node_id,
                    msg,
                } => {
                    self.flow_run(ctx, start_node_id, msg);
                }
                EngineMessage::Stop => {
                    println!("Engine stopping...");
                    break;
                }
            }
        }

        println!("Engine stopped.");
    }

    /// 核心调度逻辑
    fn flow_run(&self, mut ctx: FlowContext, start_node_id: Uuid, msg: Value) {
        let nodes: Nodes = Arc::clone(&self.nodes);
        tokio::spawn(async move {
            let mut flow_run_node_ids: VecDeque<NodeRunItem> = VecDeque::new();

            flow_run_node_ids.push_back(NodeRunItem {
                node_id: start_node_id,
                msg: msg,
            });

            while let Some(node_run_item) = flow_run_node_ids.pop_front() {
                let Some(node) = nodes.get(&node_run_item.node_id) else {
                    eprintln!("Node {} not found", node_run_item.node_id);
                    continue;
                };
                match node.on_input(&ctx, &node_run_item.msg).await {
                    Ok(node_output) => {
                        ctx.run_node_ids.push(node_run_item.node_id);
                        match node_output {
                            NodeOutput::None => continue,
                            NodeOutput::One((port, msg)) => {
                                // 获取执行node的输出节点定义
                                let out_ids = node.info().output_ports;
                                if let Some(out_node_ids) = out_ids.get(&port) {
                                    if out_node_ids.len() == 1 {
                                        let out_node_id = &out_node_ids[0];
                                        flow_run_node_ids.push_back(NodeRunItem {
                                            node_id: *out_node_id,
                                            msg: msg,
                                        });
                                    } else {
                                        for out_node_id in out_node_ids {
                                            flow_run_node_ids.push_back(NodeRunItem {
                                                node_id: *out_node_id,
                                                msg: msg.clone(),
                                            });
                                        }
                                    }
                                }
                            }
                            NodeOutput::Many(msgs) => {
                                // 获取执行node的输出节点定义
                                let out_ids = node.info().output_ports;
                                for (_, (port,msg)) in msgs.iter().enumerate() {
                                    if let Some(out_node_ids) = out_ids.get(port) {
                                        for out_node_id in out_node_ids {
                                            flow_run_node_ids.push_back(NodeRunItem {
                                                node_id: out_node_id.clone(),
                                                msg: msg.clone(),
                                            });
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
