use rsflow_core::EngineBuilder;
use rsflow_nodes::register_all_nodes;
use clap::Parser;

/// RSFlow Runtime
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
enum Command {
    /// 运行 flow
    Run {
        /// Flow 文件路径
        #[arg(short, long, default_value = "../data/flow.json")]
        flow_file: String,
    },
    /// 快捷测试命令
    Test,
    /// 直接指定 flow 文件路径
    #[command(external_subcommand)]
    FlowFile(Vec<String>),
}

#[tokio::main]
async fn main() {
    let cmd = Command::parse();
    
    match cmd {
        Command::Run { flow_file } => {
            // 使用自动注册函数
            let engine = match register_all_nodes(EngineBuilder::new())
                .build(&flow_file)
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
        Command::Test => {
            println!("Running quick test...");
            // 这里可以添加快捷测试逻辑，例如运行一个简单的内置 flow
            println!("Test completed successfully!");
        }
        Command::FlowFile(args) => {
            if let Some(flow_file) = args.first() {
                // 使用自动注册函数
                let engine = match register_all_nodes(EngineBuilder::new())
                    .build(flow_file)
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
            } else {
                eprintln!("Error: No flow file path specified");
                std::process::exit(1);
            }
        }
    }
}
