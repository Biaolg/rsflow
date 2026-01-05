use rsflow_core::EngineBuilder;
use flow_nodes::register_all_nodes;

#[tokio::main]
async fn main() {
    // 使用自动注册函数
    let engine = match register_all_nodes(EngineBuilder::new())
        .build("../data/flow.json")
        .await
    {
        Ok(engine) => engine,
        Err(e) => {
            eprintln!("Failed to build engine: {:?}", e);
            std::process::exit(1);
        }
    };

    // 👇 生命周期锚点
    engine.start().await;
}
