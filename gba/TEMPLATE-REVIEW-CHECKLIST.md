# Template Review Checklist

## 请审查以下模板

所有模板位于：`crates/gba-pm/templates/`

### 1. init.md
- [ ] 创建 .gba/.trees 目录
- [ ] 更新 .gitignore
- [ ] 指令清晰
- [ ] 英文正确

### 2. plan.md
- [ ] 移除了 description 条件判断
- [ ] 移除了 list_files() 函数
- [ ] 简化了上下文部分
- [ ] 指令清晰

### 3. phase_1_observe.md
- [ ] 简化了上下文部分
- [ ] 包含 resume_info 处理
- [ ] 指令清晰
- [ ] 输出要求明确

### 4. phase_2_build.md
- [ ] 移除了 files_to_modify/create
- [ ] 简化了 coding_standards
- [ ] 包含 resume_info 处理
- [ ] 指令清晰

### 5. phase_3_test.md
- [ ] 包含 resume_info 处理
- [ ] 测试要求清晰
- [ ] 示例代码正确
- [ ] 输出要求明确

### 6. phase_4_verification.md (NEW)
- [ ] 移除了 verification_criteria 条件判断
- [ ] 包含 resume_info 处理
- [ ] 验证清单完整
- [ ] 指令清晰

### 7. phase_5_review.md
- [ ] 简化了 coding_standards
- [ ] 包含 resume_info 处理
- [ ] 审查清单完整
- [ ] 指令清晰

### 8. phase_6_pr.md
- [ ] PR 描述模板完整
- [ ] 包含 resume_info 处理
- [ ] Git 命令正确
- [ ] 输出要求明确

## 变量检查

### 所有模板应使用的变量
- [ ] `{{ repo_path }}`
- [ ] `{{ feature_slug }}`
- [ ] `{{ specs }}`
- [ ] `{{ verification_criteria }}` (phase 4)
- [ ] `{{ previous_output }}`
- [ ] `{{ coding_standards }}`
- [ ] `{{ readme }}`
- [ ] `{% if resume_info %}` (所有 phase 模板)

### 不应出现的变量
- [ ] 没有 `{{ description }}`
- [ ] 没有 `{{ extra.* }}`
- [ ] 没有 `list_files()` 函数
- [ ] 没有 `read_file()` 函数
- [ ] 没有不必要的条件判断

## 通用检查

### 所有模板
- [ ] 全部使用英文
- [ ] 指令清晰明确
- [ ] 输出要求明确
- [ ] 示例代码正确
- [ ] 格式一致

### Phase 模板 (1-6)
- [ ] 包含 resume_info 部分
- [ ] Resume 指令清晰
- [ ] 包含 previous_output
- [ ] 包含设计规范

## 审查结果

审查人：__________
日期：__________

### 发现的问题
1.
2.
3.

### 建议的改进
1.
2.
3.

### 总体评价
- [ ] 通过，可以实施
- [ ] 需要修改后再审查
- [ ] 需要重大修改

---

**注意**：请仔细审查每个模板，确保：
1. 指令对 AI agent 来说清晰明确
2. 变量使用正确
3. 没有不必要的复杂性
4. 符合 "convention over configuration" 原则
