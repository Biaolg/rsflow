pub mod message;
pub mod sender;
pub mod flow;
pub mod node;
pub mod value;

// 为了保持向后兼容性，从旧位置重新导出
pub use message::{EngineConfig, EngineMessage};
pub use sender::EngineSender;
pub use flow::{FlowContext, FlowEventKey, FlowListeners};
pub use node::{
    Node, NodeBuilder, NodeError, NodeFactory, NodeInfo, NodeInput, NodeInputPorts, NodeOutput,
    NodeOutputPorts, NodeRunItem
};
pub use value::Value;