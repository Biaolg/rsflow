use crate::core::{EngineConfig, Value};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct FlowMod {
    pub config: EngineConfig,
    pub node_global_config: Value,
    pub flow: Vec<Flow>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Flow {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub nodes: Vec<FlowNode>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FlowNodeInputPort {
    pub port: u8,
    pub nodes: Vec<Uuid>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FlowNodeOutputPortItem {
    pub id: Uuid,
    pub port: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FlowNodeOutputPort {
    pub port: u8,
    pub nodes: Vec<FlowNodeOutputPortItem>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FlowNode {
    pub id: Uuid,
    pub name: String,
    pub node_type: String,
    pub description: String,
    pub config: Value,
    pub input: Vec<FlowNodeInputPort>,
    pub output: Vec<FlowNodeOutputPort>,
}
