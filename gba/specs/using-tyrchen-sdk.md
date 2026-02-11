# 使用 tyrchen/claude-agent-sdk-rs 的配置说明

## 方式 1: Git 依赖（推荐）

直接从 GitHub 引用：

```toml
# Cargo.toml (workspace root)
[workspace.dependencies]
# 使用 Git 仓库
claude-agent-sdk-rs = { git = "https://github.com/tyrchen/claude-agent-sdk-rs", branch = "master" }

# 或者指定特定的 tag/commit
# claude-agent-sdk-rs = { git = "https://github.com/tyrchen/claude-agent-sdk-rs", tag = "v0.6.0" }
# claude-agent-sdk-rs = { git = "https://github.com/tyrchen/claude-agent-sdk-rs", rev = "abc123" }

# 或者使用 crates.io 版本
# claude-agent-sdk-rs = "0.6"
```

然后在各个 crate 中：

```toml
# crates/gba-core/Cargo.toml
[dependencies]
claude-agent-sdk-rs = { workspace = true }
```

## 方式 2: 本地路径依赖

如果您已经克隆到本地：

```bash
# 克隆仓库到本地
cd /path/to/your/projects
git clone https://github.com/tyrchen/claude-agent-sdk-rs.git
```

然后在 `Cargo.toml` 中：

```toml
# Cargo.toml (workspace root)
[workspace.dependencies]
# 使用本地路径（相对或绝对路径）
claude-agent-sdk-rs = { path = "../claude-agent-sdk-rs" }

# 或者使用绝对路径
# claude-agent-sdk-rs = { path = "D:/projects/claude-agent-sdk-rs" }
```

## 方式 3: 混合使用（开发时用本地，发布时用 Git）

```toml
# Cargo.toml (workspace root)
[workspace.dependencies]
# 默认使用 Git
claude-agent-sdk-rs = { git = "https://github.com/tyrchen/claude-agent-sdk-rs", branch = "main" }

# 在开发时，可以在本地覆盖
# 创建 .cargo/config.toml 文件：
```

```toml
# .cargo/config.toml
[patch."https://github.com/tyrchen/claude-agent-sdk-rs"]
claude-agent-sdk-rs = { path = "../claude-agent-sdk-rs" }
```

## 注意事项

### 1. API 兼容性

如果使用 `tyrchen/claude-agent-sdk-rs`，需要更新设计文档中的代码示例，因为 API 可能不同于官方的 `claude-agent-sdk`。

### 2. 依赖冲突

确保 `tyrchen/claude-agent-sdk-rs` 的依赖版本与您的项目兼容。

### 3. 更新依赖

使用 Git 依赖时，更新方式：

```bash
# 更新到最新版本
cargo update -p claude-agent-sdk-rs

# 或者清除缓存重新下载
rm -rf ~/.cargo/git/checkouts/claude-agent-sdk-rs-*
cargo build
```

### 4. 离线使用

如果需要离线使用，本地路径依赖是最好的选择。

## 推荐方案

**对于 GBA 项目，我建议**：

### 选项 A: 使用官方 SDK（当前设计）
```toml
claude-agent-sdk = "0.1"
```
- ✅ 官方支持，稳定可靠
- ✅ 文档完善
- ✅ 长期维护
- ⚠️ 需要 Node.js 和 Claude Code CLI

### 选项 B: 使用 tyrchen 的封装
```toml
claude-agent-sdk-rs = { git = "https://github.com/tyrchen/claude-agent-sdk-rs" }
```
- ✅ 可能更轻量
- ✅ 可能不需要 Node.js
- ⚠️ 非官方，维护不确定
- ⚠️ 需要更新设计文档中的所有代码示例

## 如何切换

如果您决定使用 `tyrchen/claude-agent-sdk-rs`，需要：

1. **更新 Cargo.toml**：
```bash
# 在 workspace root
sed -i 's/claude-agent-sdk = "0.1"/claude-agent-sdk-rs = { git = "https:\/\/github.com\/tyrchen\/claude-agent-sdk-rs" }/' Cargo.toml
```

2. **更新代码导入**：
```rust
// 从
use claude_agent_sdk::{ClaudeSDKClient, ...};

// 改为
use claude_agent_sdk_rs::{...};  // 具体 API 需要查看该库的文档
```

3. **更新设计文档**：
   - 修改 `specs/design.md` 中的所有代码示例
   - 更新 API 调用方式
   - 更新系统要求

## 验证可用性

在切换之前，建议先测试该库是否可用：

```bash
# 创建测试项目
cargo new --bin test-sdk
cd test-sdk

# 添加依赖
cat >> Cargo.toml << 'EOF'
[dependencies]
claude-agent-sdk-rs = { git = "https://github.com/tyrchen/claude-agent-sdk-rs" }
tokio = { version = "1", features = ["full"] }
EOF

# 尝试编译
cargo build
```

如果编译成功，说明该库可用。然后查看其文档和示例，了解具体的 API 使用方式。
