use crate::core::{EngineSender, FlowContext, Value};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

// 节点端点定义
pub type NodeInputPorts = HashMap<u8, Vec<Uuid>>;
pub type NodeOutputPorts = HashMap<u8, Vec<(Uuid, u8)>>;

#[derive(Debug)]
pub enum NodeError {
    InvalidInput(String),
    InvalidConfig(String),
    Io(std::io::Error),
    Shell(String),
    Timeout,
    Cancelled,
}

// 节点基本信息定义
#[derive(Debug, Clone)]
pub struct NodeInfo {
    pub id: Uuid,
    pub name: String,
    pub node_type: String,
    pub description: String,
    pub config: Value,
    pub input_ports: NodeInputPorts,
    pub output_ports: NodeOutputPorts,
}

// 节点执行最小单位
#[derive(Debug)]
pub struct NodeRunItem {
    pub node_id: Uuid,
    pub node_input: NodeInput,
}

#[derive(Debug, Clone)]
pub struct NodeInput {
    pub port: u8,
    pub msg: Value,
}
//节点输出变体
pub enum NodeOutput {
    None,
    One((u8, Value)),
    Many(Vec<(u8, Value)>),
}

#[async_trait::async_trait]
pub trait Node: Send + Sync {
    /// 返回节点信息（连线、类型等）
    fn info(&self) -> NodeInfo;

    /// 初始化节点
    async fn init(&self, sender: EngineSender);

    /// 节点接收到事件时的处理
    async fn event(
        &self,
        event_type: &str,
        event_data: Value,
        ctx: &FlowContext
    ) -> Result<(), NodeError>;

    /// 节点接收到输入时的处理
    async fn input(&self,node_input: NodeInput,ctx: &FlowContext) -> Result<NodeOutput, NodeError>;
}

#[async_trait::async_trait]
pub trait NodeFactory: Send + Sync {
    async fn create(&self, node_info: NodeInfo) -> Result<Arc<dyn Node + Send + Sync>, NodeError>;
}

#[async_trait::async_trait]
pub trait NodeBuilder: Send + Sync {
    fn node_type(&self) -> &str;
    async fn register(&self) -> Result<Box<dyn NodeFactory>, NodeError>;
}
