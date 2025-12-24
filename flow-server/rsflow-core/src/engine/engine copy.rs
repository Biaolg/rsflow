use crate::core::{
    EngineMessage, EngineSender, FlowContext, Node, NodeBuilder, NodeFactory, NodeInfo, NodeOutput,
    Value,
};
use crate::flow::models::Flow;
use crate::flow::{
    FlowMod, parse_flow_all_node_types, parse_flow_all_nodes, parse_flow_file, validate_flow,
};

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

use tokio::sync::{Mutex, mpsc};
use uuid::Uuid;

type FactoryMap = HashMap<String, Box<dyn NodeFactory>>;
type NodeMap = HashMap<Uuid, Arc<dyn Node + Send + Sync>>;
type BuilderMap = HashMap<String, Box<dyn NodeBuilder>>;

pub struct Engine {
    flow_mod: FlowMod,
    nodes: NodeMap,
    receiver: Mutex<mpsc::Receiver<EngineMessage>>,
    sender: mpsc::Sender<EngineMessage>,
}

impl Engine {
    /// ⚠️ 只能通过 Builder 调用
    pub async fn create_with_builders(
        flow_file_path: &str,
        msg_len: Option<usize>,
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
                // 将 FlowNode 转换为 NodeInfo
                let outputs = flow_node
                    .output
                    .iter()
                    .map(|out| NodeOutput {
                        prot: out.prot,
                        nodes: out.nodes.clone(),
                    })
                    .collect();

                let node_info = NodeInfo {
                    id: flow_node.id,
                    name: flow_node.name.clone(),
                    node_type: flow_node.node_type.clone(),
                    description: flow_node.description.clone(),
                    config: flow_node.config.clone(),
                    input: flow_node.input.clone(),
                    output: outputs,
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

        let (tx, rx) = mpsc::channel(msg_len.unwrap_or(100));

        Ok(Arc::new(Self {
            flow_mod,
            nodes,
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
                EngineMessage::RunFlow(ctx) => {
                    self.msg_input(&ctx.start_node_id, ctx.payload).await;
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
    async fn msg_input(&self, ctx: FlowContext) {
        let mut queue = VecDeque::new();
        queue.push_back((*start_id, input));

        while let Some((node_id, input)) = queue.pop_front() {
            let Some(node) = self.nodes.get(&node_id) else {
                eprintln!("Node {} not found", node_id);
                continue;
            };

            match node.on_input(input).await {
                Ok(outputs) => {
                    println!(
                        "Node {} - {} output: {:?}",
                        node_id,
                        node.info().name,
                        outputs
                    );
                    for (idx, value) in outputs.into_iter().enumerate() {
                        if value == Value::NULL {
                            continue;
                        }
                        if let Some(out) = node.info().output.get(idx) {
                            let out_nodes = &out.nodes;

                            if out_nodes.is_empty() {
                                continue;
                            }

                            if out_nodes.len() == 1 {
                                queue.push_back((*out_nodes.first().unwrap(), value));
                                continue;
                            }

                            for next_id in out_nodes {
                                queue.push_back((*next_id, value.clone()));
                            }
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Node {} - {} error: {:?}", node_id, node.info().name, err);
                }
            }
        }
    }
}
