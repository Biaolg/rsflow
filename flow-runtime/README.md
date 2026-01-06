# RSFlow Runtime

RSFlow Runtime 是一个基于 Rust 的流式处理引擎，用于执行 flow 定义的工作流。

## 目录结构

```
flow-runtime/
├── rsflow-core/        # 核心库
├── src/                # 运行时源码
│   └── main.rs         # 主入口
├── target/             # 编译输出目录
├── Cargo.toml          # 依赖配置
└── README.md           # 本说明文件
```

## 依赖

- Rust 1.91.1 或更高版本
- Cargo 包管理器

## 编译方式

### 开发模式编译

```bash
# 在 flow-runtime 目录下执行
cargo build
```

编译产物将生成在 `target/debug/` 目录下。

### 发布模式编译

```bash
# 在 flow-runtime 目录下执行
cargo build --release
```

编译产物将生成在 `target/release/` 目录下，发布模式编译的二进制文件会进行优化，运行速度更快。

## 运行方式

### 开发模式运行

```bash
# 运行默认 flow 文件
cargo run

# 运行指定的 flow 文件
cargo run -- run -f /path/to/flow.json

# 直接指定 flow 文件路径
cargo run -- /path/to/flow.json

# 运行测试命令
cargo run -- test
```

### 二进制运行

编译完成后，可以直接运行生成的二进制文件：

```bash
# 运行默认 flow 文件
./target/debug/rsflow.exe

# 运行指定的 flow 文件
./target/debug/rsflow.exe run -f /path/to/flow.json

# 直接指定 flow 文件路径
./target/debug/rsflow.exe /path/to/flow.json

# 运行测试命令
./target/debug/rsflow.exe test
```

对于发布模式编译的二进制文件，路径为 `./target/release/rsflow.exe`。

## 命令行参数

### 主命令

```
RSFlow Runtime

Usage: rsflow.exe <COMMAND>

Commands:
  run   运行 flow
  test  快捷测试命令
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Run 子命令

```
运行 flow

Usage: rsflow.exe run [OPTIONS]

Options:
  -f, --flow-file <FLOW_FILE>  Flow 文件路径 [default: ../data/flow.json]
  -h, --help                   Print help
```

### 直接指定 flow 文件路径

```bash
rsflow.exe /path/to/flow.json
```

## 示例

### 运行默认 flow 文件

```bash
cargo run
```

### 运行指定的 flow 文件

```bash
cargo run -- run -f F:\obj\mi\rsflow\data\flow.json
```

### 直接指定 flow 文件路径

```bash
cargo run -- F:\obj\mi\rsflow\data\flow.json
```

### 运行测试命令

```bash
cargo run -- test
```

## 注意事项

1. 确保 Rust 环境已经正确安装
2. 首次运行时会自动下载依赖，可能需要一些时间
3. Flow 文件必须是有效的 JSON 格式
4. 运行时会自动注册所有节点类型

## 调试

### 查看详细日志

```bash
RUST_LOG=debug cargo run
```

### 检查编译错误

```bash
cargo check
```

## 贡献

欢迎提交 Issue 和 Pull Request 来改进 RSFlow Runtime。
