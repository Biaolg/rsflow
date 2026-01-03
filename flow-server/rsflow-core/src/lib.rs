pub mod core;
pub mod engine;
pub mod flow;

// 重新导出常用类型和接口 - 更新为新的模块路径
pub use crate::core::message::{EngineConfig, EngineMessage};
pub use crate::core::sender::EngineSender;
pub use crate::core::node::{
    Node, NodeBuilder, NodeError, NodeFactory, NodeInfo, NodeInput, NodeOutput,
    NodeRunItem,
};
pub use crate::core::flow::{FlowContext,FlowEventKey, FlowListeners};
pub use crate::core::value::Value;
pub use crate::engine::{Engine, EngineBuilder};
pub use crate::flow::{FlowMod, FlowNode, parse_flow_file, validate_flow};