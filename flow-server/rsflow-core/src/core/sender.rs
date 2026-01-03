use crate::core::{EngineMessage, FlowContext, FlowListeners, NodeRunItem};
use std::sync::Arc;
use uuid::Uuid;

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
        event_data: crate::core::Value,
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