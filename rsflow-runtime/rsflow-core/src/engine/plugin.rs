use crate::core::EngineContext;
use crate::engine::NodeBuilderMap;
#[async_trait::async_trait]
pub trait EnginePlugin: Send + Sync {
    fn name(&self) -> &'static str;
    fn internal_nodes(&self) -> NodeBuilderMap;
    async fn engine_start(&self,serde: EngineContext);
}
