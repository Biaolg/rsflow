use crate::core::{EngineConfig, Value};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct FlowMod {
    pub config: EngineConfig,
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
pub struct FlowNodeOutput {
    pub prot: u8,
    pub nodes: Vec<Uuid>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FlowNode {
    pub id: Uuid,
    pub name: String,
    pub node_type: String,
    pub description: String,
    pub config: Value,
    pub input: Vec<Uuid>,
    pub output: Vec<FlowNodeOutput>,
}
