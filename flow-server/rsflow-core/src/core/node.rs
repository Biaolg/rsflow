use crate::core::{EngineSender, FlowContext, Value};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub enum NodeError {
    InvalidInput(String),
    InvalidConfig(String),
    Io(std::io::Error),
    Shell(String),
    Timeout,
    Cancelled,
}

// 节点输出定义
pub type NodeOutputIds = HashMap<u8, Vec<Uuid>>;
// 节点消息执行最小单位
pub struct NodeRunItem {
    pub node_id: Uuid,
    pub msg: Value,
}

//节点输出变体
pub enum NodeOutput {
    None,
    One((u8, Value)),
    Many(Vec<(u8, Value)>),
}

// 节点基本信息定义
#[derive(Debug, Clone)]
pub struct NodeInfo {
    pub id: Uuid,
    pub name: String,
    pub node_type: String,
    pub description: String,
    pub config: Value,
    pub input_ids: Vec<Uuid>,
    pub output_ids: NodeOutputIds,
}

#[async_trait::async_trait]
pub trait Node: Send + Sync {
    /// 返回节点信息（连线、类型等）
    fn info(&self) -> NodeInfo;

    /// 初始化节点
    async fn on_start(&self, sender: EngineSender);

    /// 节点接收到输入时的处理
    async fn on_input(&self, ctx: &FlowContext, msg: &Value) -> Result<NodeOutput, NodeError>;
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
