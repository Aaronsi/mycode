# GBA Template Refactoring Summary

## 完成的工作

### 1. 模板结构重构 ✅

基于 claude-agent-sdk-rs 0.6 的 API 分析，我们将模板重构为两部分：

```
crates/gba-pm/templates/
├── system/                    # System Prompts (角色定义)
│   ├── base.md               # 基础系统提示词（所有阶段）
│   ├── architect.md          # 软件架构师角色（plan 阶段）
│   ├── developer.md          # 开发者角色（build 阶段）
│   ├── tester.md            # 测试工程师角色（test 阶段）
│   ├── qa.md                # QA 工程师角色（verification 阶段）
│   ├── reviewer.md          # 代码审查员角色（review 阶段）
│   └── devops.md            # DevOps 工程师角色（PR 阶段）
└── user/                     # User Prompts (任务定义)
    ├── init.md              # 初始化任务
    ├── plan.md              # 规划任务
    ├── phase_1_observe.md   # 观察任务
    ├── phase_2_build.md     # 实现任务
    ├── phase_3_test.md      # 测试任务
    ├── phase_4_verification.md  # 验证任务
    ├── phase_5_review.md    # 审查任务
    └── phase_6_pr.md        # PR 创建任务
```

### 2. SDK API 确认 ✅

从 `vendors/claude-agent-sdk-rs` 确认了 API：

```rust
// SystemPrompt 枚举定义
pub enum SystemPrompt {
    Text(String),              // 自定义文本提示词
    Preset(SystemPromptPreset), // 预设提示词
}

// SystemPromptPreset 结构
pub struct SystemPromptPreset {
    pub preset: String,        // 预设名称（如 "claude_code"）
    pub append: Option<String>, // 追加文本
}

// 使用方式
let options = ClaudeAgentOptions {
    system_prompt: Some(SystemPrompt::Text("自定义提示词".to_string())),
    ..Default::default()
};

// 或使用预设 + 追加
let options = ClaudeAgentOptions {
    system_prompt: Some(SystemPrompt::Preset(
        SystemPromptPreset::with_append("claude_code", "额外指令")
    )),
    ..Default::default()
};
```

### 3. System Prompts 创建 ✅

创建了 7 个专业角色的 system prompt：

1. **base.md**: 基础系统提示词
   - AI 身份和能力
   - 工具使用规则
   - 代码质量标准
   - 安全准则

2. **architect.md**: 软件架构师
   - 架构设计专长
   - 需求分析
   - 技术规划
   - 风险评估

3. **developer.md**: 软件开发者
   - Rust 编程专长
   - 类型驱动开发
   - 异步编程
   - API 设计

4. **tester.md**: 测试工程师
   - 测试设计和策略
   - 单元和集成测试
   - 属性测试
   - 测试组织

5. **qa.md**: 质量保证工程师
   - 需求验证
   - 验收测试
   - 质量指标
   - 合规检查

6. **reviewer.md**: 代码审查员
   - 代码质量分析
   - 架构审查
   - 安全审查
   - 性能审查

7. **devops.md**: DevOps 工程师
   - Git 工作流
   - 提交规范
   - PR 管理
   - 发布管理

### 4. User Prompts 简化 ✅

简化了所有 user prompt，移除了角色定义部分：

**之前**（角色和任务混合）：
```markdown
# Phase 2: Build Implementation

You are an expert Rust developer with deep knowledge of...

## Your Expertise
- Rust programming
- Type-driven development
...

## Your Task
Implement the feature...
```

**之后**（只包含任务）：
```markdown
# Phase 2: Build Implementation

## Feature: {{ feature_slug }}
**Repository**: {{ repo_path }}

## Your Task
Implement the feature according to the design specification...
```

### 5. 配置结构更新

需要更新 `.gba/config.yaml` 以支持分离的提示词：

```yaml
version: "0.1.0"
claude_api_key_env: "ANTHROPIC_API_KEY"
default_model: "claude-sonnet-4-5"
timeout_seconds: 300

phases:
  - name: "observe"
    system_prompt: "system/base.md"
    user_prompt: "user/phase_1_observe.md"
    description: "Observe codebase and understand context"

  - name: "build"
    system_prompt: "system/developer.md"
    user_prompt: "user/phase_2_build.md"
    description: "Build implementation"

  - name: "test"
    system_prompt: "system/tester.md"
    user_prompt: "user/phase_3_test.md"
    description: "Write and run tests"

  - name: "verification"
    system_prompt: "system/qa.md"
    user_prompt: "user/phase_4_verification.md"
    description: "Verify implementation against requirements"

  - name: "review"
    system_prompt: "system/reviewer.md"
    user_prompt: "user/phase_5_review.md"
    description: "Code review and refinement"

  - name: "pr"
    system_prompt: "system/devops.md"
    user_prompt: "user/phase_6_pr.md"
    description: "Create pull request"
```

## 实现建议

### 代码实现

```rust
// 在 gba-core/src/agent.rs 中

use claude_agent_sdk_rs::{ClaudeClient, ClaudeAgentOptions, SystemPrompt};

pub async fn execute_phase(
    phase_config: &PhaseConfig,
    context: &PromptContext,
) -> Result<ExecutionResult> {
    // 1. 加载并渲染 system prompt
    let system_template = load_template(&phase_config.system_prompt)?;
    let system_prompt_text = render_template(&system_template, context)?;

    // 2. 加载并渲染 user prompt
    let user_template = load_template(&phase_config.user_prompt)?;
    let user_prompt_text = render_template(&user_template, context)?;

    // 3. 创建 ClaudeAgentOptions
    let options = ClaudeAgentOptions {
        system_prompt: Some(SystemPrompt::Text(system_prompt_text)),
        model: Some("claude-sonnet-4-5".to_string()),
        max_turns: Some(50),
        ..Default::default()
    };

    // 4. 创建客户端并执行
    let mut client = ClaudeClient::new(options);
    client.connect().await?;
    client.query(&user_prompt_text).await?;

    // 5. 接收响应
    let mut stream = client.receive_response();
    while let Some(result) = stream.next().await {
        // 处理响应...
    }

    client.disconnect().await?;
    Ok(execution_result)
}
```

### 配置结构更新

```rust
// 在 gba-pm/src/config.rs 中

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PhaseConfig {
    pub name: String,
    pub system_prompt: String,  // 新增：system prompt 模板路径
    pub user_prompt: String,    // 新增：user prompt 模板路径
    pub description: String,

    // 向后兼容：如果只有 prompt_template，则作为 user_prompt
    #[serde(default)]
    pub prompt_template: Option<String>,
}

impl PhaseConfig {
    pub fn get_user_prompt(&self) -> &str {
        if !self.user_prompt.is_empty() {
            &self.user_prompt
        } else if let Some(ref template) = self.prompt_template {
            template
        } else {
            panic!("No user prompt specified")
        }
    }
}
```

## 优势

### 1. 清晰的关注点分离
- System prompt: 定义 AI 的角色和能力
- User prompt: 定义具体任务和上下文

### 2. 更好的可维护性
- 角色定义独立维护
- 任务描述独立更新
- 不会相互干扰

### 3. 更高的灵活性
- 可以为不同阶段使用不同角色
- 可以混合搭配角色和任务
- 支持自定义角色

### 4. 符合最佳实践
- 遵循 prompt engineering 最佳实践
- 充分利用 SDK 的 system prompt 功能
- 更好的 AI 行为控制

## 下一步

1. **更新 design.md**：添加 system/user prompt 分离的设计说明
2. **实现代码**：在 gba-core 中实现 system prompt 支持
3. **更新配置**：修改默认 config.yaml 模板
4. **测试验证**：测试不同角色的 system prompt 效果
5. **文档更新**：更新用户文档说明新的模板结构

## 文件清单

### 新增文件
- `crates/gba-pm/templates/system/base.md`
- `crates/gba-pm/templates/system/architect.md`
- `crates/gba-pm/templates/system/developer.md`
- `crates/gba-pm/templates/system/tester.md`
- `crates/gba-pm/templates/system/qa.md`
- `crates/gba-pm/templates/system/reviewer.md`
- `crates/gba-pm/templates/system/devops.md`
- `crates/gba-pm/templates/user/init.md`
- `crates/gba-pm/templates/user/plan.md`
- `crates/gba-pm/templates/user/phase_1_observe.md`
- `crates/gba-pm/templates/user/phase_2_build.md`
- `crates/gba-pm/templates/user/phase_3_test.md`
- `crates/gba-pm/templates/user/phase_4_verification.md`
- `crates/gba-pm/templates/user/phase_5_review.md`
- `crates/gba-pm/templates/user/phase_6_pr.md`
- `crates/gba-pm/templates/STRUCTURE.md`

### 需要更新的文件
- `specs/design.md` - 添加 system/user prompt 分离说明
- `crates/gba-pm/src/config.rs` - 更新 PhaseConfig 结构
- `crates/gba-core/src/agent.rs` - 实现 system prompt 支持
- `.gba/config.yaml` (模板) - 更新配置格式

---

**状态**: ✅ 模板重构完成
**下一步**: 实现代码支持和测试验证
