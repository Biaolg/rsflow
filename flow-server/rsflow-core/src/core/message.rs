use crate::core::{FlowContext, NodeRunItem, Payload};
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
        payload: Payload,
    },
    Stop,
}