use std::collections::HashMap;
use crate::core::node::NodeBuilder;

type BuilderMap = HashMap<String, Box<dyn NodeBuilder>>;

pub struct EngineBuilder {
    builders: BuilderMap,
}

impl EngineBuilder {
    pub fn new() -> Self {
        Self {
            builders: HashMap::new(),
        }
    }

    /// 注册一个节点类型（插件）
    pub fn register_node<B>(mut self, builder: B) -> Self
    where
        B: NodeBuilder + 'static,
    {
        self.builders
            .insert(builder.node_type().to_string(), Box::new(builder));
        self
    }

    /// 构建 Engine
    pub async fn build(
        self,
        flow_file_path: &str,
        msg_len: Option<usize>,
    ) -> std::result::Result<std::sync::Arc<crate::engine::engine::Engine>, std::io::Error> {
        crate::engine::engine::Engine::create_with_builders(
            flow_file_path,
            msg_len,
            self.builders,
        )
        .await
    }
}
