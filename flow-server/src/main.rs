mod nodes;
mod auto_node_registry;

use rsflow_core::EngineBuilder;

#[tokio::main]
async fn main() {
    // 使用自动注册函数
    let engine = auto_node_registry::register_all_nodes_to_builder(EngineBuilder::new())
        .build("./data/flow.json", Some(100))
        .await
        .expect("Failed to build engine");

    // 👇 生命周期锚点
    engine.start().await;
}
