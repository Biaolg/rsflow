use rsflow_core::{
    EngineSender, FlowContext, Node, NodeBuilder, NodeError, NodeFactory, NodeInfo, NodeInput,
    NodeOutput, Payload, Value,
};
use std::sync::Arc;
use tokio::process::Command;
use tokio::time::{Duration, timeout};

// Node 实例
pub struct ShellNode {
    info: NodeInfo,
    shell_type: &'static str,
    timeout: Duration,
}

fn shell_args(shell: &str, command: &str) -> Vec<String> {
    if shell == "cmd" {
        vec!["/C".to_string(), command.to_string()]
    } else {
        vec!["-c".to_string(), command.to_string()]
    }
}

#[async_trait::async_trait]
impl Node for ShellNode {
    fn info(&self) -> NodeInfo {
        self.info.clone()
    }
    async fn engine_start(&self, _: EngineSender) {
        // 初始化逻辑
    }
    async fn event(&self, _: &str, _: Payload, _: &FlowContext) -> Result<(), NodeError> {
        Ok(())
    }
    async fn input(&self, node_input: NodeInput, _: &FlowContext) -> Result<NodeOutput, NodeError> {
        // 从 input 或 config 获取 command
        let command_str = match node_input.msg {
            Payload {
                value: Value::Object(map),
                ..
            } => map.get("command").and_then(|v| match v {
                Value::String(s) => Some(s.clone()),
                _ => None,
            }),
            _ => None,
        }
        .or_else(|| match &self.info.config {
            Value::String(s) => Some(s.clone()),
            Value::Object(map) => map.get("command").and_then(|v| match v {
                Value::String(s) => Some(s.clone()),
                _ => None,
            }),
            _ => None,
        })
        .ok_or_else(|| {
            NodeError::InvalidConfig("Missing command in input or config".to_string())
        })?;

        let args = shell_args(self.shell_type, &command_str);

        let mut cmd = Command::new(self.shell_type);
        for a in &args {
            cmd.arg(a);
        }

        // 设置超时
        let output = timeout(self.timeout, cmd.output())
            .await
            .map_err(|_| NodeError::Timeout)?
            .map_err(NodeError::Io)?;

        if !output.status.success() {
            return Err(NodeError::Shell(format!(
                "Shell error: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(NodeOutput::One((
            0,
            Payload::new(Value::String(
                String::from_utf8_lossy(&output.stdout).to_string(),
            )),
        )))
    }
}

// NodeFactory
pub struct ShellNodeFactory;

#[async_trait::async_trait]
impl NodeFactory for ShellNodeFactory {
    async fn create(&self, node_info: NodeInfo) -> Result<Arc<dyn Node + Send + Sync>, NodeError> {
        // 从配置中获取超时设置，默认5秒
        let timeout_seconds = match &node_info.config {
            Value::Object(map) => map.get("timeout").and_then(|v| match v {
                Value::Int(i) => Some(*i as u64),
                Value::Long(l) => Some(*l as u64),
                _ => None,
            }),
            _ => None,
        }
        .unwrap_or(5);

        Ok(Arc::new(ShellNode {
            info: node_info,
            shell_type: if cfg!(windows) { "cmd" } else { "sh" },
            timeout: Duration::from_secs(timeout_seconds),
        }))
    }
}

// NodeBuilder
pub struct ShellNodeBuilder;

#[async_trait::async_trait]
impl NodeBuilder for ShellNodeBuilder {
    fn node_type(&self) -> &str {
        "shell"
    }

    async fn register(&self, _: &Value) -> Result<Box<dyn NodeFactory>, NodeError> {
        Ok(Box::new(ShellNodeFactory))
    }
}
