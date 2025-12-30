use rsflow_core::{
    EngineContext, FlowContext, Node, NodeBuilder, NodeError, NodeFactory, NodeInfo, NodeInput,
    NodeOutput, Value,
};
use std::sync::Arc;

pub struct LogNode {
    info: NodeInfo,
}

#[async_trait::async_trait]
impl Node for LogNode {
    fn info(&self) -> NodeInfo {
        self.info.clone()
    }
    async fn init(&self, _: EngineContext) {}
    async fn event(&self, _: &str, _: Value, _: &FlowContext) -> Result<(), NodeError> {
        Ok(())
    }
    async fn input(
        &self,
        node_input: NodeInput,
        ctx: &FlowContext,
    ) -> Result<NodeOutput, NodeError> {
        println!("{} {:#?}", ctx.id, node_input.msg);
        Ok(NodeOutput::None)
    }
}

// NodeFactory 负责创建 Node
pub struct LogNodeFactory;

#[async_trait::async_trait]
impl NodeFactory for LogNodeFactory {
    async fn create(&self, node_info: NodeInfo) -> Result<Arc<dyn Node + Send + Sync>, NodeError> {
        Ok(Arc::new(LogNode { info: node_info }))
    }
}

// NodeBuilder 负责注册插件
pub struct LogNodeBuilder;

#[async_trait::async_trait]
impl NodeBuilder for LogNodeBuilder {
    fn node_type(&self) -> &str {
        "log"
    }

    async fn register(&self) -> Result<Box<dyn NodeFactory>, NodeError> {
        // 返回一个 LogNodeFactory 的实例
        Ok(Box::new(LogNodeFactory))
    }
}
