use crate::core::{EngineMessage, FlowContext, NodeRunItem, Payload};
use crate::flow::FlowMod;
use std::sync::Arc;
use uuid::Uuid;

pub struct EngineContext {
    pub flow_mod:Arc<FlowMod>,
    pub sender: tokio::sync::mpsc::Sender<EngineMessage>,
}

impl EngineContext {
    pub async fn run_flow(&self, start_node: NodeRunItem) {
        let ctx = FlowContext::new(Uuid::new_v4());
        let _ = self
            .sender
            .send(EngineMessage::RunFlow { ctx, start_node })
            .await;
    }
    
    pub async fn node_send(
        &self,
        node_id: Uuid,
        ctx: FlowContext,
        event_type: String,
        payload: Payload,
    ) {
        let _ = self
            .sender
            .send(EngineMessage::NodeEvent {
                node_id,
                ctx,
                event_type,
                payload,
            })
            .await;
    }
}