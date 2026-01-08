use crate::core::{NodeBuilder, NodeFactory, NodeInfo, NodeInputPorts, NodeOutputPorts,Value};
use crate::flow::{FlowMod, FlowNode, parse_flow_all_nodes, parse_flow_file, validate_flow};

use std::collections::{HashMap, HashSet};
use std::io::Error as IoError;
use std::io::ErrorKind;
use uuid::Uuid;

// 类型别名
type FactoryMap = HashMap<String, Box<dyn NodeFactory>>;
type BuilderMap = HashMap<String, Box<dyn NodeBuilder>>;

/// 流程处理器，负责解析和处理流程配置，创建节点实例
pub struct FlowProcessor;

impl FlowProcessor {
    /// 解析流程文件并验证
    pub fn parse_flow_file(file_path: &str) -> Result<FlowMod, IoError> {
        // 解析 flow
        let flow_mod = parse_flow_file(file_path)?;

        // 验证 flow 配置
        if let Err(err) = validate_flow(&flow_mod) {
            return Err(IoError::new(
                ErrorKind::InvalidData,
                format!("Flow validation failed: {}", err),
            ));
        }

        Ok(flow_mod)
    }

    /// 从流程模型中提取所有节点
    pub fn extract_nodes(flow_mod: &FlowMod) -> Vec<FlowNode> {
        parse_flow_all_nodes(flow_mod.clone())
    }

    /// 从节点列表中提取所有节点类型
    pub fn extract_node_types(rsflow_nodes: &[FlowNode]) -> HashSet<String> {
        rsflow_nodes
            .iter()
            .map(|node| node.node_type.clone())
            .collect()
    }

    /// 将节点构建器转换为节点工厂
    pub async fn builders_to_factories(
        builders: BuilderMap,
        node_types: &HashSet<String>,
        node_global_config: &Value
    ) -> Result<FactoryMap, IoError> {
        let mut factories: FactoryMap = HashMap::new();
        let mut missing_builders = Vec::new();

        for t in node_types {
            if let Some(builder) = builders.get(t) {
                match builder.register(node_global_config).await {
                    Ok(factory) => {
                        factories.insert(t.clone(), factory);
                    }
                    Err(err) => {
                        return Err(IoError::new(
                            ErrorKind::Other,
                            format!("Failed to register factory for node type {}: {:?}", t, err),
                        ));
                    }
                }
            } else {
                missing_builders.push(t.clone());
            }
        }

        if !missing_builders.is_empty() {
            return Err(IoError::new(
                ErrorKind::InvalidData,
                format!("Missing builders for node types: {:?}", missing_builders),
            ));
        }

        Ok(factories)
    }

    /// 从流程节点创建核心节点实例
    pub async fn create_nodes_from_flow(
        rsflow_nodes: Vec<FlowNode>,
        factories: &FactoryMap,
        node_global_config: &Value,
    ) -> Result<HashMap<Uuid, std::sync::Arc<dyn crate::core::Node + Send + Sync>>, IoError> {
        let mut nodes = HashMap::new();

        for flow_node in rsflow_nodes {
            if let Some(factory) = factories.get(&flow_node.node_type) {
                // 将 FlowNodeInput 转换为 NodeInputPorts
                let inputs: NodeInputPorts = flow_node
                    .input
                    .iter()
                    .map(|in_| (in_.port, in_.nodes.clone()))
                    .collect();

                // 将 FlowNode 转换为 NodeInfo
                let outputs: NodeOutputPorts = flow_node
                    .output
                    .iter()
                    .map(|out| {
                        (
                            out.port,
                            out.nodes
                                .clone()
                                .into_iter()
                                .map(|item| (item.id, item.port))
                                .collect(),
                        )
                    })
                    .collect();

                let global_config = match node_global_config {
                    Value::Object(map) => map.get(&flow_node.node_type).cloned(),
                    _ => None,
                }; 

                let node_info = NodeInfo {
                    id: flow_node.id,
                    name: flow_node.name.clone(),
                    node_type: flow_node.node_type.clone(),
                    description: flow_node.description.clone(),
                    config: flow_node.config.clone(),
                    input_ports: inputs,
                    output_ports: outputs,
                    global_config: global_config.unwrap_or(Value::NULL),
                };

                match factory.create(node_info).await {
                    Ok(node) => {
                        nodes.insert(flow_node.id, node);
                    }
                    Err(err) => {
                        return Err(IoError::new(
                            ErrorKind::Other,
                            format!("Failed to create node {}: {:?}", flow_node.id, err),
                        ));
                    }
                }
            }
        }

        Ok(nodes)
    }
}
