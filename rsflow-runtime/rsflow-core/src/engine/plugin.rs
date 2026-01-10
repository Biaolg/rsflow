use crate::engine::NodeBuilderMap;

#[async_trait::async_trait]
pub trait EnginePlugin: Send + Sync {
    fn name(&self) -> &'static str;
    async fn engine_start(&mut self) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
pub trait EnginePluginBuilder: Send + Sync {
    fn plugin_name(&self) -> &str;
    fn internal_nodes(&self) -> NodeBuilderMap;
    async fn build(self: Box<Self>) -> anyhow::Result<Box<dyn EnginePlugin>>;
}
