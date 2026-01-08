use crate::core::node::NodeBuilder;

#[async_trait::async_trait]
pub trait EnginePlugin: Send + Sync {
    fn name(&self) -> &'static str;
}

#[async_trait::async_trait]
pub trait EnginePluginBuilder: Send + Sync {
    fn plugin_name(&self) -> &str;
    fn internal_node(&self) -> Vec<Box<dyn NodeBuilder>>;
    async fn build(self: Box<Self>) -> anyhow::Result<Box<dyn EnginePlugin>>;
}
