use rsflow_core::{EngineSender, FlowContext, Node, NodeBuilder, NodeError, NodeFactory, NodeInfo, NodeInput, NodeOutput, Payload};
use std::sync::Arc;

pub struct HttpOutNode {
    info: NodeInfo,
}

#[async_trait::async_trait]
impl Node for HttpOutNode {
    fn info(&self) -> NodeInfo {
        self.info.clone()
    }
    async fn init(&self, sender: EngineSender) { }

    async fn event(&self, _: &str, _: Payload, _: &FlowContext) -> Result<(), NodeError> {
        Ok(())
    }

    async fn input(
        &self,
        node_input: NodeInput,
        _: &FlowContext,
    ) -> Result<NodeOutput, NodeError> {
        Ok(NodeOutput::One((0, node_input.msg)))
    }
}

// NodeFactory 负责创建 Node
pub struct HttpOutNodeFactory;

#[async_trait::async_trait]
impl NodeFactory for HttpOutNodeFactory {
    async fn create(&self, node_info: NodeInfo) -> Result<Arc<dyn Node + Send + Sync>, NodeError> {
        Ok(Arc::new(HttpOutNode { info: node_info }))
    }
}

// NodeBuilder 负责注册插件
pub struct HttpOutNodeBuilder;

#[async_trait::async_trait]
impl NodeBuilder for HttpOutNodeBuilder {
    fn node_type(&self) -> &str {
        "http_out"
    }

    async fn register(&self) -> Result<Box<dyn NodeFactory>, NodeError> {
        // 返回一个 HttpOutNodeFactory 的实例
        Ok(Box::new(HttpOutNodeFactory))
    }
}
