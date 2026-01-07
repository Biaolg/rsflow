use crate::core::{EngineSender, FlowContext, Payload, ResourceId, StreamId, Value};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

// ===== 基础类型定义 =====
// 节点端点定义
pub type NodeInputPorts = HashMap<u8, Vec<Uuid>>;
pub type NodeOutputPorts = HashMap<u8, Vec<(Uuid, u8)>>;

// ===== 数据结构 =====
// 节点执行最小单位
#[derive(Debug)]
pub struct NodeRunItem {
    pub node_id: Uuid,
    pub node_input: NodeInput,
}

#[derive(Debug, Clone)]
pub struct NodeInput {
    pub port: u8,
    pub msg: Payload,
}

// 节点输出变体
pub enum NodeOutput {
    None,
    One((u8, Payload)),
    Many(Vec<(u8, Payload)>),
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
    pub global_config: Value,
}

// ===== 错误处理 =====
#[derive(Debug)]
pub enum NodeError {
    InvalidInput(String),
    InvalidConfig(String),
    Io(std::io::Error),
    Shell(String),
    Timeout,
    Cancelled,
    ResourceNotFound(ResourceId),
    StreamNotFound(StreamId),
}

// ===== 核心功能trait =====
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
        payload: Payload,
        ctx: &FlowContext,
    ) -> Result<(), NodeError>;

    /// 节点接收到输入时的处理
    async fn input(
        &self,
        node_input: NodeInput,
        ctx: &FlowContext,
    ) -> Result<NodeOutput, NodeError>;
}

// ===== 工厂模式trait =====
#[async_trait::async_trait]
pub trait NodeFactory: Send + Sync {
    async fn create(&self, node_info: NodeInfo) -> Result<Arc<dyn Node + Send + Sync>, NodeError>;
}

#[async_trait::async_trait]
pub trait NodeBuilder: Send + Sync {
    fn node_type(&self) -> &str;
    async fn register(&self, global_config: &Value) -> Result<Box<dyn NodeFactory>, NodeError>;
}
