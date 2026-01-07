use rsflow_core::{
    EngineSender, FlowContext, Node, NodeBuilder, NodeError, NodeFactory, NodeInfo, NodeInput,
    NodeOutput, Payload, Value,
};
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::OnceLock;

static ADDR: OnceLock<SocketAddr> = OnceLock::new();

pub struct HttpInNode {
    info: NodeInfo
}

#[async_trait::async_trait]
impl Node for HttpInNode {
    fn info(&self) -> NodeInfo {
        self.info.clone()
    }
    async fn init(&self, _: EngineSender) {}

    async fn event(&self, _: &str, _: Payload, _: &FlowContext) -> Result<(), NodeError> {
        Ok(())
    }

    async fn input(&self, node_input: NodeInput, _: &FlowContext) -> Result<NodeOutput, NodeError> {
        Ok(NodeOutput::One((0, node_input.msg)))
    }
}

// NodeFactory 负责创建 Node
pub struct HttpInNodeFactory;

#[async_trait::async_trait]
impl NodeFactory for HttpInNodeFactory {
    //创建节点实例
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

    async fn register(
        &self,
        node_global_config: &Value,
    ) -> Result<Box<dyn NodeFactory>, NodeError> {
        //创建端口监听
        let port = match Some(node_global_config) {
            Some(Value::Object(map)) => map
                .get("port")
                .and_then(|v| match v {
                    Value::Int(i) => Some(*i as u16),
                    Value::Long(l) => Some(*l as u16),
                    _ => None,
                })
                .unwrap_or(8080),
            _ => 8080,
        };
        ADDR.set(SocketAddr::from(([127, 0, 0, 1], port))).unwrap();
        // 返回一个 HttpInNodeFactory 的实例
        Ok(Box::new(HttpInNodeFactory))
    }
}
