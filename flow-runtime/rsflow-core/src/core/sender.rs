use crate::core::{EngineMessage, FlowContext, NodeRunItem, Payload};
use uuid::Uuid;

pub struct EngineSender {
    pub tx: tokio::sync::mpsc::Sender<EngineMessage>,
}

impl EngineSender {
    pub async fn run_flow(&self, start_node: NodeRunItem) {
        let ctx = FlowContext::new(Uuid::new_v4());
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
        payload: Payload,
    ) {
        let _ = self
            .tx
            .send(EngineMessage::NodeEvent {
                node_id,
                ctx,
                event_type,
                payload,
            })
            .await;
    }
}