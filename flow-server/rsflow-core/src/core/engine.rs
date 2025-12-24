use crate::core::value::Value;
use uuid::Uuid;
// 前向声明 EngineSender，避免循环依赖
pub struct EngineSender {
    pub tx: tokio::sync::mpsc::Sender<EngineMessage>,
}

impl EngineSender {
    pub async fn run_flow(&self, ctx: FlowContext) {
        let _ = self.tx.send(EngineMessage::RunFlow(ctx)).await;
    }
    pub async fn send(&self, node_id: Uuid, input: Value) {
        let ctx = FlowContext{
            id: Uuid::new_v4(),
            start_node_id: node_id,
            payload: input,
        };
        let _ = self.tx.send(EngineMessage::RunFlow(ctx)).await;
    }
}

// flow上下文
#[derive(Clone, Debug)]
pub struct FlowContext {
    pub id: Uuid,
    pub start_node_id: Uuid,
    pub payload: Value,
}

// 引擎消息定义
#[derive(Clone, Debug)]
pub enum EngineMessage {
    RunFlow(FlowContext),
    Stop,
}
