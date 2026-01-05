pub use flow_node_inject::InjectNodeBuilder;
pub use flow_node_log::LogNodeBuilder;
pub use flow_node_net::{HttpInNodeBuilder, HttpOutNodeBuilder};
pub use flow_node_shell::ShellNodeBuilder;

/// 注册所有节点到 EngineBuilder
pub fn register_all_nodes(builder: rsflow_core::EngineBuilder) -> rsflow_core::EngineBuilder {
    let builder = builder.register_node(InjectNodeBuilder {});
    let builder = builder.register_node(LogNodeBuilder {});
    let builder = builder.register_node(HttpInNodeBuilder {});
    let builder = builder.register_node(HttpOutNodeBuilder {});
    let builder = builder.register_node(ShellNodeBuilder {});
    builder
}
