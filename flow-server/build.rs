use std::fs;
use std::path::Path;
use std::collections::HashSet;

fn main() {
    println!("cargo:rerun-if-changed=src/nodes/");
    println!("cargo:rerun-if-changed=rsflow-core/");
    
    let nodes_dir = Path::new("src/nodes");
    let mut node_modules = HashSet::new();
    
    // 扫描nodes目录下的所有模块文件
    if let Ok(entries) = fs::read_dir(nodes_dir) {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".rs") && file_name != "mod.rs" {
                    let module_name = &file_name[0..file_name.len()-3];
                    node_modules.insert(module_name.to_string());
                }
            }
        }
    }
    
    // 生成自动注册代码
    let mut code = String::from("use rsflow_core::EngineBuilder;\n\n");
    code.push_str("/// 自动注册所有节点到EngineBuilder\n");
    code.push_str("pub fn register_all_nodes_to_builder(mut builder: EngineBuilder) -> EngineBuilder {\n");
    
    for module in node_modules {
        code.push_str(&format!("    // 注册 {} 节点\n", module));
        code.push_str(&format!("    use crate::nodes::{}::*;\n", module));
        
        // 尝试获取模块中的构建器类型（通过约定的命名规则）
    let builder_type = module
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new()
            }
        })
        .collect::<String>();
    let builder_type = format!("{}NodeBuilder", builder_type);
        code.push_str(&format!("    builder = builder.register_node({} {{}});\n\n", builder_type));
    }
    
    code.push_str("    builder\n");
    code.push_str("}\n");
    
    // 写入生成的代码到OUT_DIR
    let out_dir = match std::env::var("OUT_DIR") {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Failed to get OUT_DIR environment variable: {}", e);
            return;
        }
    };
    let out_path = Path::new(&out_dir).join("auto_node_registry.rs");
    if let Err(e) = fs::write(&out_path, code) {
        eprintln!("Failed to write auto_node_registry.rs to {}: {}", out_path.display(), e);
        return;
    }
    
    // 确保cargo知道这个文件
    match out_path.to_str() {
        Some(path_str) => println!("cargo:rerun-if-changed={}", path_str),
        None => eprintln!("Failed to convert output path to string: {:?}", out_path),
    }
}
