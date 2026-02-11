# Claude Agent SDK 说明

## 使用的 SDK

本项目使用 **tyrchen 的 Rust SDK 实现**：

- **Crate 名称**: `claude-agent-sdk-rs`
- **版本**: 0.6.4+
- **仓库**: https://github.com/tyrchen/claude-agent-sdk-rs
- **依赖方式**: Git 依赖
- **文档**: https://docs.rs/claude-agent-sdk-rs
- **许可证**: MIT

## 为什么选择这个 SDK？

1. **纯 Rust 实现**: 无需 Node.js 或 Claude Code CLI
2. **直接 API 调用**: 直接调用 Anthropic API，无中间层
3. **功能完整**: 双向流式支持、会话管理、自定义工具
4. **类型安全**: 100% 安全 Rust 代码，Rust 2024 edition
5. **异步优先**: 基于 tokio 的完整异步支持
6. **丰富示例**: 包含 24 个示例代码

## 架构说明

`claude-agent-sdk-rs` 直接调用 Anthropic API：

```
GBA Application
    ↓
claude-agent-sdk-rs (Rust)
    ↓
Anthropic API (HTTPS)
```

### 工作原理

1. **直接 HTTP 调用**: 通过 HTTPS 直接调用 Anthropic API
2. **双向流**: 支持流式请求和响应
3. **会话管理**: SDK 管理会话状态和上下文
4. **无外部依赖**: 不需要 Node.js 或其他运行时

## 系统要求

### 必需组件

1. **Rust**: 1.90.0 或更高版本（2024 edition）
2. **Tokio**: 异步运行时（已包含在依赖中）

### 环境变量

需要设置 Anthropic API 密钥：
```bash
export ANTHROPIC_API_KEY="your-api-key"
```

## 核心 API

### 1. ClaudeClient（推荐用于 GBA）

用于多轮对话和有状态交互：

```rust
use claude_agent_sdk_rs::{ClaudeClient, ClaudeAgentOptions};

let options = ClaudeAgentOptions::builder()
    .api_key("your-api-key")
    .model("claude-sonnet-4-5")
    .max_turns(10)
    .build()?;

let mut client = ClaudeClient::new(options)?;
client.connect().await?;
client.query("Hello!").await?;

let mut stream = client.receive_response();
while let Some(message) = stream.next().await {
    // Process messages
}

client.disconnect().await?;
```

### 2. query() 函数

用于简单的一次性查询：

```rust
use claude_agent_sdk_rs::query;

let messages = query("What is 2 + 2?", None).await?;
for message in messages {
    // Process messages
}
```

### 3. query_stream() 函数

用于内存高效的流式查询：

```rust
use claude_agent_sdk_rs::query_stream;
use futures::StreamExt;

let mut stream = query_stream("Explain Rust", None).await?;
while let Some(message) = stream.next().await {
    // Process messages
}
```

### 4. 消息类型

```rust
pub enum Message {
    Assistant(Vec<ContentBlock>),
    User(Vec<ContentBlock>),
    System(String),
    Result(ConversationResult),
}

pub enum ContentBlock {
    Text(TextBlock),
    ToolUse(ToolUseBlock),
    ToolResult(ToolResultBlock),
}

pub struct TextBlock {
    pub text: String,
}
```

## GBA 中的使用

在 GBA 项目中，我们使用 `ClaudeClient` 来：

1. **执行计划阶段**: 与用户交互式对话，生成功能计划
2. **执行实现阶段**: 按阶段执行任务（observe, build, test, review, PR）
3. **流式输出**: 实时显示 Agent 的执行进度
4. **错误恢复**: 处理超时和重试

### 示例：GBA 中的使用

```rust
// gba-core/src/runner.rs
pub struct AgentRunner {
    options: ClaudeAgentOptions,
}

impl AgentRunner {
    pub async fn execute(&self, prompt: &str) -> Result<String> {
        let mut client = ClaudeClient::new(self.options.clone())?;
        client.connect().await?;
        client.query(prompt).await?;

        let mut output = String::new();
        let mut stream = client.receive_response();

        while let Some(message) = stream.next().await {
            if let Ok(Message::Assistant(content)) = message {
                for block in content {
                    if let ContentBlock::Text(TextBlock { text }) = block {
                        output.push_str(&text);
                    }
                }
            } else if let Ok(Message::Result(_)) = message {
                break;
            }
        }

        client.disconnect().await?;
        Ok(output)
    }
}
```

## 与其他 SDK 的对比

### claude-agent-sdk-rs (本项目使用)
- ✅ tyrchen 维护
- ✅ 直接调用 Anthropic API
- ✅ 纯 Rust 实现，无需 Node.js
- ✅ 完整的 Agent 功能（工具调用、会话管理）
- ✅ 双向流式支持
- ✅ 24 个示例代码

### claude-agent-sdk (Anthropic 官方)
- ✅ Anthropic 官方
- ✅ 封装 Claude Code CLI
- ✅ 完整的 Agent 功能
- ⚠️ 需要 Node.js 和 Claude Code CLI
- ⚠️ 多层封装，可能有性能开销

### anthropic-sdk-rust (直接 API)
- ✅ 直接调用 Anthropic API
- ✅ 不需要 Node.js
- ❌ 没有 Agent 功能（需要自己实现工具调用）
- ❌ 没有会话管理

## 配置方式

在 `Cargo.toml` 中使用 Git 依赖：

```toml
[workspace.dependencies]
claude-agent-sdk-rs = { git = "https://github.com/tyrchen/claude-agent-sdk-rs", branch = "master" }
```

或者使用 crates.io 版本：

```toml
[workspace.dependencies]
claude-agent-sdk-rs = "0.6"
```

## 参考资料

- [claude-agent-sdk-rs 文档](https://docs.rs/claude-agent-sdk-rs)
- [GitHub 仓库](https://github.com/tyrchen/claude-agent-sdk-rs)
- [Crates.io](https://crates.io/crates/claude-agent-sdk-rs)

## 故障排除

### 问题：API 密钥未设置

```
Error: ANTHROPIC_API_KEY environment variable not set
```

**解决方案**:
```bash
export ANTHROPIC_API_KEY="your-api-key"
```

### 问题：网络超时

```
Error: Request timeout
```

**解决方案**: 检查网络连接，或在 `ClaudeAgentOptions` 中增加超时时间。

### 问题：Git 依赖下载失败

```
Error: failed to fetch from git repository
```

**解决方案**: 检查网络连接，或使用本地路径依赖：
```bash
git clone https://github.com/tyrchen/claude-agent-sdk-rs.git
```
```toml
claude-agent-sdk-rs = { path = "../claude-agent-sdk-rs" }
```
