# GBA 设计文档更新完成

## 完成时间
2026-02-11

## 更新内容

### 1. 添加了新的 Section 3.2.4: Template Organization and System Prompts

详细说明了新的模板组织结构：

#### 模板结构
```
crates/gba-pm/templates/
├── init/          (system.md + user.md)
├── plan/          (system.md + user.md)
├── observe/       (system.md + user.md)
├── build/         (system.md + user.md)
├── test/          (system.md + user.md)
├── verification/  (system.md + user.md)
├── review/        (system.md + user.md)
└── pr/            (system.md + user.md)
```

#### System Prompt vs User Prompt
- **System Prompt**: 定义 AI 角色、专长和行为
- **User Prompt**: 定义具体任务和上下文

#### 专业角色
1. Base role (init, observe)
2. Architect role (plan)
3. Developer role (build)
4. Tester role (test)
5. QA role (verification)
6. Reviewer role (review)
7. DevOps role (pr)

#### SDK 集成示例
```rust
use claude_agent_sdk_rs::{ClaudeAgentOptions, SystemPrompt};

let (system_prompt, user_prompt) = prompt_manager.load_phase_prompts("build", &context)?;

let options = ClaudeAgentOptions {
    system_prompt: Some(SystemPrompt::Text(system_prompt)),
    ..Default::default()
};
```

### 2. 更新了配置格式 (Section 4.2)

#### 旧格式
```yaml
phases:
  - name: "build"
    prompt_template: "phase_2_build.md"
    description: "Build implementation"
```

#### 新格式（约定优于配置）
```yaml
# Templates are automatically loaded from {phase_name}/system.md and {phase_name}/user.md
phases:
  - name: "build"
    description: "Build implementation"
    # Loads: build/system.md + build/user.md
```

### 3. 核心优势

1. **清晰的关注点分离**: 角色定义与任务描述分离
2. **可复用性**: System prompt 可跨任务共享
3. **易于维护**: 独立更新角色或任务
4. **灵活性**: 混合搭配不同角色和任务
5. **最佳实践**: 遵循 prompt engineering 标准
6. **精细控制**: 每个阶段的 AI 行为可精确控制

### 4. 约定优于配置

- 模板自动从 `{phase_name}/system.md` 和 `{phase_name}/user.md` 加载
- 配置中只需指定 phase name，无需指定模板路径
- 简化配置，减少维护成本

## Git 提交记录

```
5d5c7e7 - Update design.md with new template structure
626d248 - Add template reorganization completion summary
2af9803 - Reorganize templates by step with system.md and user.md
db4128a - Refactor templates to separate system and user prompts
11c9470 - Add comprehensive GBA design documentation and templates
```

## 文档更新位置

### design.md 中的更新

1. **Section 3.2.4** (新增): Template Organization and System Prompts
   - 位置: Line 1112
   - 内容: 完整的模板结构说明、SDK 集成、角色定义

2. **Section 4.2** (更新): Configuration File
   - 位置: Line 1340+
   - 内容: 简化的配置格式，移除 prompt_template 字段

## 相关文档

1. **specs/template-refactoring-summary.md** - 重构总结
2. **specs/template-reorganization-complete.md** - 重组完成总结
3. **crates/gba-pm/templates/STRUCTURE.md** - 模板结构文档

## 实现指南

### PromptManager 需要实现的新方法

```rust
impl PromptManager {
    /// Load both system and user prompts for a phase
    pub fn load_phase_prompts(
        &self,
        phase_name: &str,
        context: &PromptContext,
    ) -> Result<(String, String)> {
        let system_path = format!("{}/system.md", phase_name);
        let user_path = format!("{}/user.md", phase_name);

        let system_prompt = self.render(&system_path, context)?;
        let user_prompt = self.render(&user_path, context)?;

        Ok((system_prompt, user_prompt))
    }
}
```

### AgentRunner 需要更新

```rust
impl AgentRunner {
    pub async fn execute_phase(
        &self,
        phase_name: &str,
        context: &PromptContext,
    ) -> Result<ExecutionResult> {
        // Load both prompts
        let (system_prompt, user_prompt) =
            self.prompt_manager.load_phase_prompts(phase_name, context)?;

        // Create options with system prompt
        let options = ClaudeAgentOptions {
            system_prompt: Some(SystemPrompt::Text(system_prompt)),
            model: Some("claude-sonnet-4-5".to_string()),
            ..Default::default()
        };

        // Execute
        let mut client = ClaudeClient::new(options);
        client.connect().await?;
        client.query(&user_prompt).await?;

        // ... handle response
    }
}
```

## 下一步工作

1. ✅ 更新 design.md - **已完成**
2. ⏳ 实现 PromptManager::load_phase_prompts()
3. ⏳ 更新 AgentRunner 使用新的 API
4. ⏳ 更新 PhaseConfig 结构（移除 prompt_template）
5. ⏳ 测试新的模板加载逻辑
6. ⏳ 更新用户文档

## 总结

✅ **design.md 更新完成**

成功在设计文档中添加了新的模板结构说明，包括：
- 详细的模板组织方式
- System prompt 和 user prompt 的区别
- 7 个专业角色的定义
- SDK 集成示例
- 约定优于配置的实现方式
- 配置格式的简化

设计文档现在完整反映了新的模板架构，为实现提供了清晰的指导。

---

**完成日期**: 2026-02-11
**提交**: 5d5c7e7
**状态**: ✅ 完成
