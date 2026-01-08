use crate::flow::{FlowMod, FlowNode};
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufReader};

// 解析flow.json文件
pub fn parse_flow_file(file_path: &str) -> Result<FlowMod, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let flow_mod = serde_json::from_reader(reader).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("JSON parse error: {}", e),
        )
    })?;
    Ok(flow_mod)
}

// 获取所有节点
pub fn parse_flow_all_nodes(flow_mod: FlowMod) -> Vec<FlowNode> {
    let mut nodes = Vec::new();
    for flow_item in flow_mod.flow {
        nodes.extend(flow_item.nodes);
    }
    nodes
}

// 获取所有节点类型
pub fn parse_flow_all_node_types(rsflow_nodes: Vec<FlowNode>) -> HashSet<String> {
    rsflow_nodes.into_iter().map(|node| node.node_type).collect()
}

// 验证flow配置
pub fn validate_flow(flow_mod: &FlowMod) -> Result<(), String> {
    // 检查是否有重复节点ID
    let mut node_ids = HashSet::new();
    for flow in &flow_mod.flow {
        for node in &flow.nodes {
            if !node_ids.insert(node.id) {
                return Err(format!("Duplicate node ID found: {}", node.id));
            }
        }
    }

    // 检查输出连接的节点是否存在
    let all_node_ids: HashSet<_> = node_ids;
    for flow in &flow_mod.flow {
        for node in &flow.nodes {
            for output in &node.output {
                for next_item in &output.nodes {
                    if !all_node_ids.contains(&next_item.id) {
                        return Err(format!(
                            "Node {} references non-existent node {}",
                            node.id, next_item.id
                        ));
                    }
                }
            }
        }
    }

    Ok(())
}
