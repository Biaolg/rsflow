use crate::core::{FlowContext, NodeRunItem, Value};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct EngineConfig {
    pub msg_len: usize,
}

// 前向声明 EngineSender，避免循环依赖
pub struct EngineSender {
    pub tx: tokio::sync::mpsc::Sender<EngineMessage>,
}

impl EngineSender {
    pub async fn run_flow(&self, start_node: NodeRunItem) {
        let ctx = FlowContext {
            id: Uuid::new_v4(),
            run_node_ids: vec![],
        };
        let _ = self
            .tx
            .send(EngineMessage::RunFlow {
                ctx,
                start_node,
            })
            .await;
    }
}

// 引擎消息定义
#[derive(Clone, Debug)]
pub enum EngineMessage {
    RunFlow {
        ctx: FlowContext,
        start_node: NodeRunItem,
    },
    Stop,
}
