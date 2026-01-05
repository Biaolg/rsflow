use rsflow_core::{EngineSender, FlowContext, Node, NodeBuilder, NodeError, NodeFactory, NodeInfo, NodeInput, NodeOutput, Payload, Value};
use std::sync::Arc;

pub struct HttpInNode {
    info: NodeInfo,
}

#[async_trait::async_trait]
impl Node for HttpInNode {
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
pub struct HttpInNodeFactory;

#[async_trait::async_trait]
impl NodeFactory for HttpInNodeFactory {
    async fn create(&self, node_info: NodeInfo) -> Result<Arc<dyn Node + Send + Sync>, NodeError> {
        Ok(Arc::new(HttpInNode { info: node_info }))
    }
}

// NodeBuilder 负责注册插件
pub struct HttpInNodeBuilder;

#[async_trait::async_trait]
impl NodeBuilder for HttpInNodeBuilder {
    fn node_type(&self) -> &str {
        "http_in"
    }

    async fn register(&self) -> Result<Box<dyn NodeFactory>, NodeError> {
        // 返回一个 HttpInNodeFactory 的实例
        Ok(Box::new(HttpInNodeFactory))
    }
}
