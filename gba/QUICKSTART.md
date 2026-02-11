# GBA 项目快速参考

## 项目状态

✅ **配置完成** - 所有文件已更新为使用 `claude-agent-sdk-rs`
✅ **编译通过** - 使用 Rust nightly 1.95.0
✅ **文档完整** - 设计文档和 API 说明已更新

## 快速开始

### 环境要求

- **Rust**: nightly (自动通过 rust-toolchain.toml 配置)
- **环境变量**: `ANTHROPIC_API_KEY`

### 编译和运行

```bash
# 检查编译
cargo check

# 构建项目
cargo build

# 运行 CLI
cargo run -p gba-cli -- --help

# 运行测试
cargo test

# 代码格式化
cargo fmt

# 代码检查
cargo clippy
```

## 核心技术栈

| 组件 | 技术 | 说明 |
|------|------|------|
| **gba-core** | tokio + claude-agent-sdk-rs | 执行引擎 |
| **gba-pm** | minijinja | 提示词管理 |
| **gba-cli** | clap + ratatui | 命令行界面 |

## 使用的 SDK

**claude-agent-sdk-rs** v0.6.3
- 仓库: https://github.com/tyrchen/claude-agent-sdk-rs
- 文档: https://docs.rs/claude-agent-sdk-rs
- 特点: 纯 Rust，无需 Node.js，直接调用 Anthropic API

## 核心 API 示例

```rust
use claude_agent_sdk_rs::{ClaudeClient, ClaudeAgentOptions};

// 创建客户端
let options = ClaudeAgentOptions::builder()
    .api_key(api_key)
    .model("claude-sonnet-4-5")
    .max_turns(10)
    .build()?;

let mut client = ClaudeClient::new(options)?;

// 连接并查询
client.connect().await?;
client.query("Hello!").await?;

// 接收流式响应
let mut stream = client.receive_response();
while let Some(message) = stream.next().await {
    match message? {
        Message::Assistant(content) => {
            // 处理响应
        }
        Message::Result(_) => break,
        _ => {}
    }
}

client.disconnect().await?;
```

## 项目结构

```
gba/
├── apps/
│   └── gba-cli/           # CLI 应用
├── crates/
│   ├── gba-core/          # 核心引擎
│   └── gba-pm/            # 提示词管理
├── specs/                 # 设计文档
│   ├── design.md          # 主设计文档 ⭐
│   ├── claude-sdk-notes.md # SDK 说明
│   └── FINAL-UPDATE-SUMMARY.md # 更新总结
├── Cargo.toml             # Workspace 配置
└── rust-toolchain.toml    # Rust nightly 配置
```

## 重要文档

1. **[specs/design.md](specs/design.md)** - 完整设计文档
   - 系统架构
   - 三个 crate 的详细设计
   - API 示例和工作流

2. **[specs/claude-sdk-notes.md](specs/claude-sdk-notes.md)** - SDK 使用指南
   - API 文档
   - 使用示例
   - 故障排除

3. **[specs/FINAL-UPDATE-SUMMARY.md](specs/FINAL-UPDATE-SUMMARY.md)** - 更新总结
   - 所有变更详情
   - API 对比
   - 验证结果

## 开发计划

### Phase 1: 基础设施（第 1 周）
- [ ] 设置 CI/CD
- [ ] 实现错误类型
- [ ] 创建基础结构

### Phase 2: gba-pm（第 1-2 周）
- [ ] 实现 PromptManager
- [ ] 模板缓存
- [ ] 单元测试

### Phase 3: gba-core（第 2-3 周）
- [ ] 实现 AgentRunner
- [ ] Actor 模型
- [ ] 集成测试

### Phase 4-6: gba-cli（第 3-5 周）
- [ ] init 命令
- [ ] plan 命令
- [ ] run 命令

### Phase 7: 完善（第 6 周）
- [ ] 文档
- [ ] 优化
- [ ] 发布

## 常见命令

```bash
# 更新依赖
cargo update

# 清理构建
cargo clean

# 查看依赖树
cargo tree

# 运行特定 crate 的测试
cargo test -p gba-core

# 构建发布版本
cargo build --release
```

## 环境变量

```bash
# 必需
export ANTHROPIC_API_KEY="your-api-key"

# 可选（日志级别）
export RUST_LOG=debug
```

## 故障排除

### 问题：API 密钥未设置
```bash
export ANTHROPIC_API_KEY="your-api-key"
```

### 问题：Rust 版本不对
```bash
# 项目会自动使用 nightly（通过 rust-toolchain.toml）
cd gba
cargo check  # 自动安装并使用 nightly
```

### 问题：编译错误
```bash
# 清理并重新构建
cargo clean
cargo check
```

## 资源链接

- [Rust 文档](https://doc.rust-lang.org/)
- [Tokio 文档](https://tokio.rs)
- [claude-agent-sdk-rs 文档](https://docs.rs/claude-agent-sdk-rs)
- [Clap 文档](https://docs.rs/clap)
- [Ratatui 文档](https://docs.rs/ratatui)

---

**版本**: 1.1 (使用 claude-agent-sdk-rs)
**最后更新**: 2026-02-10
**状态**: ✅ 就绪，可以开始开发
