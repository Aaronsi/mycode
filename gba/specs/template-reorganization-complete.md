# GBA 模板重构完成总结

## 完成时间
2026-02-11

## 重构内容

### 1. 模板结构重组 ✅

**从**：
```
templates/
├── system/
│   ├── base.md
│   ├── architect.md
│   ├── developer.md
│   └── ...
└── user/
    ├── init.md
    ├── plan.md
    └── phase_*.md
```

**到**：
```
templates/
├── init/
│   ├── system.md
│   └── user.md
├── plan/
│   ├── system.md
│   └── user.md
├── observe/
│   ├── system.md
│   └── user.md
├── build/
│   ├── system.md
│   └── user.md
├── test/
│   ├── system.md
│   └── user.md
├── verification/
│   ├── system.md
│   └── user.md
├── review/
│   ├── system.md
│   └── user.md
└── pr/
    ├── system.md
    └── user.md
```

### 2. SDK API 确认 ✅

从 `vendors/claude-agent-sdk-rs` 确认了 API 支持：

```rust
pub enum SystemPrompt {
    Text(String),              // 自定义文本
    Preset(SystemPromptPreset), // 预设 + 追加
}

// 使用方式
let options = ClaudeAgentOptions {
    system_prompt: Some(SystemPrompt::Text(custom_text)),
    ..Default::default()
};
```

### 3. 配置格式更新

**新的 config.yaml 格式**：
```yaml
phases:
  - name: "build"
    system_prompt: "build/system.md"
    user_prompt: "build/user.md"
    description: "Build implementation"
```

**更简洁**：
- 每个步骤只需指定目录名
- system.md 和 user.md 是固定文件名
- 易于理解和维护

## 优势

### 1. 组织清晰
- 按步骤组织，一目了然
- 每个步骤自包含
- 易于导航和查找

### 2. 配置简单
- 只需指定步骤目录
- 文件名固定（system.md, user.md）
- 减少配置复杂度

### 3. 维护方便
- 修改某个步骤的提示词，直接进入对应目录
- system 和 user 分离，互不干扰
- 添加新步骤只需创建新目录

### 4. 符合直觉
- 开发者按步骤思考
- 目录结构反映执行流程
- 易于理解和使用

## 实现指南

### 代码实现

```rust
// 在 gba-core/src/agent.rs 中

pub async fn execute_phase(
    phase_name: &str,
    context: &PromptContext,
) -> Result<ExecutionResult> {
    // 1. 构建模板路径
    let system_path = format!("{}/system.md", phase_name);
    let user_path = format!("{}/user.md", phase_name);

    // 2. 加载并渲染
    let system_prompt = load_and_render(&system_path, context)?;
    let user_prompt = load_and_render(&user_path, context)?;

    // 3. 创建选项
    let options = ClaudeAgentOptions {
        system_prompt: Some(SystemPrompt::Text(system_prompt)),
        ..Default::default()
    };

    // 4. 执行
    let mut client = ClaudeClient::new(options);
    client.connect().await?;
    client.query(&user_prompt).await?;

    // 5. 处理响应
    // ...

    Ok(result)
}
```

### 配置加载

```rust
// 在 gba-pm/src/config.rs 中

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PhaseConfig {
    pub name: String,
    pub system_prompt: String,  // "build/system.md"
    pub user_prompt: String,    // "build/user.md"
    pub description: String,
}
```

## Git 提交记录

1. **db4128a**: Refactor templates to separate system and user prompts
   - 创建 system/ 和 user/ 目录
   - 分离角色定义和任务描述

2. **2af9803**: Reorganize templates by step with system.md and user.md
   - 重组为按步骤的目录结构
   - 每个步骤包含 system.md 和 user.md

## 文件清单

### 新增目录和文件
- `templates/init/system.md` - 初始化系统提示词
- `templates/init/user.md` - 初始化用户提示词
- `templates/plan/system.md` - 规划系统提示词（架构师角色）
- `templates/plan/user.md` - 规划用户提示词
- `templates/observe/system.md` - 观察系统提示词
- `templates/observe/user.md` - 观察用户提示词
- `templates/build/system.md` - 构建系统提示词（开发者角色）
- `templates/build/user.md` - 构建用户提示词
- `templates/test/system.md` - 测试系统提示词（测试工程师角色）
- `templates/test/user.md` - 测试用户提示词
- `templates/verification/system.md` - 验证系统提示词（QA 角色）
- `templates/verification/user.md` - 验证用户提示词
- `templates/review/system.md` - 审查系统提示词（审查员角色）
- `templates/review/user.md` - 审查用户提示词
- `templates/pr/system.md` - PR 系统提示词（DevOps 角色）
- `templates/pr/user.md` - PR 用户提示词

### 更新文件
- `templates/STRUCTURE.md` - 更新为新的结构说明
- `specs/template-refactoring-summary.md` - 重构总结文档

### 删除文件
- `templates/system/*` - 旧的系统提示词目录
- `templates/user/*` - 旧的用户提示词目录
- `templates/init.md` - 旧的扁平模板文件
- `templates/plan.md` - 旧的扁平模板文件
- `templates/phase_*.md` - 旧的扁平模板文件

## 下一步工作

### 1. 更新设计文档
- [ ] 在 `specs/design.md` 中添加新的模板结构说明
- [ ] 更新配置文件格式说明
- [ ] 添加 system prompt 使用示例

### 2. 实现代码支持
- [ ] 更新 `gba-pm/src/config.rs` 中的 PhaseConfig
- [ ] 实现 `gba-core/src/agent.rs` 中的 system prompt 加载
- [ ] 更新模板渲染逻辑

### 3. 更新默认配置
- [ ] 修改 `init.md` 模板中的 config.yaml 生成
- [ ] 使用新的配置格式

### 4. 测试验证
- [ ] 测试每个步骤的 system prompt 效果
- [ ] 验证模板渲染正确性
- [ ] 测试完整的执行流程

### 5. 文档更新
- [ ] 更新用户文档
- [ ] 添加模板自定义指南
- [ ] 更新 README

## 技术要点

### System Prompt 的作用
- 定义 AI 的角色和专长
- 设置工作原则和标准
- 提供工具使用指南
- 建立安全和质量准则

### User Prompt 的作用
- 描述具体任务
- 提供上下文信息
- 定义输出要求
- 给出执行步骤

### 分离的好处
- 角色定义可复用
- 任务描述更清晰
- 维护更容易
- 符合最佳实践

## 总结

✅ **模板重构完成**

新的按步骤组织的结构更加清晰、易用和可维护。每个步骤都有专门的 system.md（角色定义）和 user.md（任务描述），充分利用了 claude-agent-sdk-rs 的 SystemPrompt API。

这个结构为 GBA 的实现提供了坚实的基础，使得 AI agent 能够在不同阶段扮演不同的专业角色，提供更高质量的输出。

---

**完成日期**: 2026-02-11
**提交记录**: db4128a, 2af9803
**状态**: ✅ 完成
