use crate::core::{EngineSender, Value, FlowContext};
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
#[derive(Debug, Clone)]
pub struct NodeOutput {
    pub prot: u8,
    pub nodes: Vec<Uuid>,
}

// 节点基本信息定义
#[derive(Debug, Clone)]
pub struct NodeInfo {
    pub id: Uuid,
    pub name: String,
    pub node_type: String,
    pub description: String,
    pub config: Value,
    pub input: Vec<Uuid>,
    pub output: Vec<NodeOutput>,
}

#[async_trait::async_trait]
pub trait Node: Send + Sync {
    /// 返回节点信息（连线、类型等）
    fn info(&self) -> NodeInfo;

    /// 初始化节点
    async fn on_start(&self, sender: EngineSender);

    /// 节点接收到输入时的处理
    async fn on_input(&self,ctx:FlowContext) -> Result<Vec<FlowContext>, NodeError>;
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
