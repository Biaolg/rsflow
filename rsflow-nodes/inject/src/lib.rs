use rsflow_core::{EngineSender, FlowContext, Node, NodeBuilder, NodeError, NodeFactory, NodeInfo, NodeInput, NodeOutput, NodeRunItem, Payload, Value};
use std::sync::Arc;

pub struct InjectNode {
    info: NodeInfo,
}

#[async_trait::async_trait]
impl Node for InjectNode {
    fn info(&self) -> NodeInfo {
        self.info.clone()
    }
    async fn init(&self, sender: EngineSender) {
        let node_id = self.info.id;

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));

            loop {
                interval.tick().await;
                sender
                    .run_flow(NodeRunItem {
                        node_id,
                        node_input: NodeInput {
                            port: 0,
                            msg: Payload::new(Value::String("hello".to_string())),
                        },
                    })
                    .await; // ✅ 使用复制的值
            }
        });
    }

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

    async fn register(&self, _: &Value) -> Result<Box<dyn NodeFactory>, NodeError> {
        // 返回一个 InjectNodeFactory 的实例
        Ok(Box::new(InjectNodeFactory))
    }
}
