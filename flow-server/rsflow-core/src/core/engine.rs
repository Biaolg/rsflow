use crate::core::{FlowContext, NodeRunItem, Value};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct EngineConfig {
    pub msg_len: usize,
}

// 引擎消息定义
#[derive(Debug)]
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

pub struct EngineContext {
    pub tx: tokio::sync::mpsc::Sender<EngineMessage>,
}

impl EngineContext {
    pub async fn run_flow(&self, start_node: NodeRunItem) {
        let ctx = FlowContext {
            id: Uuid::new_v4(),
            run_node_ids: vec![],
        };
        let _ = self
            .tx
            .send(EngineMessage::RunFlow { ctx, start_node })
            .await;
    }
}
