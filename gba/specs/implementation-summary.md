# 设计文档更新总结

## 重要说明：Claude Agent SDK

**使用的 SDK**: `claude-agent-sdk-rs` (v0.6.4+)
- **仓库**: https://github.com/tyrchen/claude-agent-sdk-rs
- **依赖方式**: Git 依赖
- **文档**: https://docs.rs/claude-agent-sdk-rs

**注意**: 这是 tyrchen 的 Rust SDK 实现，直接调用 Anthropic API。

**系统要求**:
- Rust 1.90.0+ (2024 edition)
- Tokio async runtime
- 环境变量: `ANTHROPIC_API_KEY`

**优势**:
- ✅ 纯 Rust 实现，无需 Node.js
- ✅ 直接调用 Anthropic API
- ✅ 双向流式支持
- ✅ 会话管理和自定义工具
- ✅ 24 个示例代码

## 已完成的更新

### 1. 详细的 gba-core 实现说明

在设计文档的 **3.1 gba-core** 章节中，新增了以下详细内容：

#### 3.1.1 Claude Agent SDK Integration
- 展示了如何使用 `claude-agent-sdk-rs` 0.6.4+ 的具体代码
- `AgentRunner` 的内部实现，包括 API 配置和消息发送
- 超时处理和错误恢复机制
- 使用 `ClaudeClient` 进行双向流式通信

#### 3.1.2 Actor-Based Execution Model
- 完整的 Actor 模型实现，使用 `tokio::sync::mpsc` 通道
- `AgentExecutorActor` 和 `AgentExecutorHandle` 的设计
- 使用 `AtomicBool` 实现优雅关闭
- 消息传递机制（Execute 和 Shutdown 消息）

#### 3.1.3 Phase Execution with Tokio
- 多阶段顺序执行的异步实现
- 使用 `tokio::spawn` 管理并发任务
- 阶段间上下文传递机制
- 错误处理和失败恢复

#### 3.1.4 Streaming Output Support
- 实时流式输出支持
- 使用 `tokio::sync::broadcast` 广播进度更新
- 适用于 CLI 实时显示执行进度

#### 3.1.5 Error Handling and Retry Logic
- 指数退避重试机制
- 使用 `tokio::time::sleep` 实现延迟
- 可重试错误的判断逻辑（网络错误、限流、超时）

### 2. 详细的 gba-pm 实现说明

在设计文档的 **3.2 gba-pm** 章节中，新增了以下详细内容：

#### 3.2.1 Minijinja Integration
- 完整的 `PromptManager` 实现，使用 `minijinja::Environment`
- 模板加载器配置（`path_loader`）
- 自定义过滤器：`slugify` 和 `indent`
- 自定义函数：`read_file` 和 `list_files`
- 使用 `parking_lot::RwLock` 实现高效缓存

#### 3.2.2 Template Context Builder
- 流式 API 设计（builder pattern）
- 缓存键生成机制
- 上下文序列化支持

#### 3.2.3 Template Examples
- 完整的 `plan.md` 模板示例
- 完整的 `phase_2_build.md` 模板示例
- 展示了 Jinja2 语法的实际应用：
  - 条件判断 `{% if %}`
  - 循环 `{% for %}`
  - 过滤器 `| indent(4)`
  - 函数调用 `read_file()`, `list_files()`

### 3. 依赖项更新

#### Workspace Cargo.toml
- 升级 edition 到 `2024`
- 添加 `rust-version = "1.83"`
- 新增依赖：
  - `parking_lot = "0.12"` - 高性能锁
  - `dashmap = "6.1"` - 并发 HashMap
  - `glob = "0.3"` - 文件模式匹配
  - `tracing = "0.1"` - 结构化日志
  - `tracing-subscriber = "0.3"` - 日志订阅器
- 扩展 tokio features：添加 `sync` 和 `time`

#### gba-core Cargo.toml
- 添加 `tracing`, `parking_lot`, `dashmap`

#### gba-pm Cargo.toml
- 添加 `parking_lot`, `glob`, `tracing`

#### gba-cli Cargo.toml
- 添加 `tracing`, `tracing-subscriber`

### 4. 新增文件

#### rust-toolchain.toml
- 固定 Rust 版本为 1.83
- 包含 rustfmt 和 clippy 组件
- 使用 minimal profile

## 关键设计原则

### gba-core
1. **Actor 模型**：每个 agent 执行都在独立的 actor 中，通过消息传递通信
2. **异步优先**：所有 I/O 操作使用 tokio 异步运行时
3. **优雅关闭**：使用 AtomicBool 和 Shutdown 消息实现
4. **错误恢复**：指数退避重试机制处理瞬态故障
5. **流式输出**：支持实时进度更新

### gba-pm
1. **模板缓存**：使用 RwLock 实现高效并发缓存
2. **自定义扩展**：提供过滤器和函数扩展 Jinja2 功能
3. **路径加载器**：自动发现和加载模板文件
4. **上下文构建器**：流式 API 简化上下文创建
5. **快速失败**：模板验证在加载时进行

## 技术栈总结

| 组件 | 核心技术 | 用途 |
|------|---------|------|
| gba-core | tokio + claude-agent-sdk-rs 0.6 | 异步执行引擎 |
| gba-pm | minijinja + parking_lot | 模板管理和渲染 |
| gba-cli | clap + ratatui | 命令行界面 |
| 并发 | tokio::sync (mpsc, broadcast, oneshot) | Actor 消息传递 |
| 日志 | tracing + tracing-subscriber | 结构化日志 |
| 错误处理 | thiserror + anyhow | 类型安全的错误 |

**重要**: `claude-agent-sdk-rs` 是 tyrchen 的 Rust SDK，直接调用 Anthropic API，提供：
- `ClaudeClient` - 双向流式客户端
- `query()` / `query_stream()` - 简单查询接口
- 完整的流式支持
- Hook 系统和权限管理
- MCP 服务器集成
- 无需 Node.js 和 Claude Code CLI

## 下一步

设计文档已完整，可以开始实施开发计划：

1. **Phase 1**: 设置基础架构和错误类型
2. **Phase 2**: 实现 gba-pm（提示词管理器）
3. **Phase 3**: 实现 gba-core（执行引擎）
4. **Phase 4-6**: 实现 gba-cli 的三个命令
5. **Phase 7**: 完善文档和测试

所有代码将遵循 CLAUDE.md 中的最佳实践。
