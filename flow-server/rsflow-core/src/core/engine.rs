use crate::core::{FlowContext, Value, node};
use uuid::Uuid;
// 前向声明 EngineSender，避免循环依赖
pub struct EngineSender {
    pub tx: tokio::sync::mpsc::Sender<EngineMessage>,
}

impl EngineSender {
    pub async fn run_flow(&self, ctx: FlowContext, start_node_id: Uuid, msg: Value) {
        let _ = self
            .tx
            .send(EngineMessage::RunFlow {
                ctx,
                start_node_id,
                msg,
            })
            .await;
    }
    pub async fn send(&self, node_id: Uuid, msg: Value) {
        let ctx = FlowContext {
            id: Uuid::new_v4(),
            run_node_ids: vec![],
        };
        let _ = self
            .tx
            .send(EngineMessage::RunFlow {
                ctx,
                start_node_id: node_id,
                msg,
            })
            .await;
    }
}

// 引擎消息定义
#[derive(Clone, Debug)]
pub enum EngineMessage {
    RunFlow {
        ctx: FlowContext,
        start_node_id: Uuid,
        msg: Value,
    },
    Stop,
}
