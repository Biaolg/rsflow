pub use flow_node_inject::InjectNodeBuilder;
pub use flow_node_log::LogNodeBuilder;
pub use flow_node_shell::ShellNodeBuilder;

/// 注册所有节点到 EngineBuilder
pub fn register_all_nodes(builder: rsflow_core::EngineBuilder) -> rsflow_core::EngineBuilder {
    builder
        .register_node(InjectNodeBuilder {})
        .register_node(LogNodeBuilder {})
        .register_node(ShellNodeBuilder {})
}
