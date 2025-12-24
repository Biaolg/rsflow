use rsflow_core::{EngineSender, FlowContext, Node, NodeBuilder, NodeError, NodeFactory, NodeInfo, Value};
use std::sync::Arc;

pub struct InjectNode {
    info: NodeInfo,
}

#[async_trait::async_trait]
impl Node for InjectNode {
    fn info(&self) -> NodeInfo {
        self.info.clone()
    }
    async fn on_start(&self, sender: EngineSender) {
        // tokio::spawn(async move {
        //     let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));

        //     loop {
        //         interval.tick().await;
        //         sender.send(self.info.id, Value::Int(1)).await;
        //     }
        // });
    }
    async fn on_input(&self, ctx: FlowContext) -> Result<Vec<FlowContext>, NodeError> {
        Ok(vec![ctx])
    }
}

// NodeFactory 负责创建 Node
pub struct InjectNodeFactory;

#[async_trait::async_trait]
impl NodeFactory for InjectNodeFactory {
    async fn create(&self, node_info: NodeInfo) -> Result<Arc<dyn Node + Send + Sync>, NodeError> {
        Ok(Arc::new(InjectNode { info: node_info }))
    }
}

// NodeBuilder 负责注册插件
pub struct InjectNodeBuilder;

#[async_trait::async_trait]
impl NodeBuilder for InjectNodeBuilder {
    fn node_type(&self) -> &str {
        "inject"
    }

    async fn register(&self) -> Result<Box<dyn NodeFactory>, NodeError> {
        // 返回一个 InjectNodeFactory 的实例
        Ok(Box::new(InjectNodeFactory))
    }
}
