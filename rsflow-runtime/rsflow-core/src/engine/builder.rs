use crate::core::node::NodeBuilder;
use crate::engine::plugin::{EnginePluginBuilder};
use std::collections::HashMap;

pub type NodeBuilderMap = HashMap<String, Box<dyn NodeBuilder>>;
pub type PluginBuilderMap = HashMap<String, Box<dyn EnginePluginBuilder>>;

pub struct EngineBuilder {
    nodes: NodeBuilderMap,
    plugins: PluginBuilderMap,
}

impl EngineBuilder {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            plugins: HashMap::new(),
        }
    }

    /// 注册一个节点类型（插件）
    pub fn register_node<B>(mut self, builder: B) -> Self
    where
        B: NodeBuilder + 'static,
    {
        self.nodes
            .insert(builder.node_type().to_string(), Box::new(builder));
        self
    }

    /// 注册中引擎插件
    pub fn register_engine_plugin<B>(mut self, builder: B) -> Self
    where
        B: EnginePluginBuilder + 'static,
    {
        self.plugins
            .insert(builder.plugin_name().to_string(), Box::new(builder));
        self
    }

    /// 构建 Engine
    pub async fn build(
        self,
        flow_file_path: &str,
    ) -> std::result::Result<std::sync::Arc<crate::engine::engine::Engine>, std::io::Error> {
        crate::engine::engine::Engine::create_with_builders(
            flow_file_path,
            self.nodes,
            self.plugins,
        )
        .await
    }
}
