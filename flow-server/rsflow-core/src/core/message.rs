use crate::core::{FlowContext, NodeRunItem, Value};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct EngineConfig {
    pub msg_len: usize,
}

pub enum EngineMessage {
    RunFlow {
        ctx: FlowContext,
        start_node: NodeRunItem,
    },
    NodeEvent {
        node_id: Uuid,
        ctx: FlowContext,
        event_type: String,
        event_data: Value,
    },
    Stop,
}