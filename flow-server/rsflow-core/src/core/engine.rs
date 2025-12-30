use crate::core::{FlowContext, FlowListeners, NodeRunItem, Value};
use serde::Deserialize;
use std::sync::Arc;
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

pub struct EngineSender {
    pub tx: tokio::sync::mpsc::Sender<EngineMessage>,
}

impl EngineSender {
    pub async fn run_flow(&self, start_node: NodeRunItem) {
        let ctx = FlowContext {
            id: Uuid::new_v4(),
            run_node_ids: vec![],
            listeners: Arc::new(FlowListeners::new()),
        };
        let _ = self
            .tx
            .send(EngineMessage::RunFlow { ctx, start_node })
            .await;
    }
    pub async fn node_send(
        &self,
        node_id: Uuid,
        ctx: FlowContext,
        event_type: String,
        event_data: Value,
    ) {
        let _ = self
            .tx
            .send(EngineMessage::NodeEvent {
                node_id,
                ctx,
                event_type,
                event_data,
            })
            .await;
    }
}
