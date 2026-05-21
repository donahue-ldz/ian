# Spec: Dialogue Life Context

## 状态

已实现，待验收。

## 问题 / 目标

Ian 的短句需要和当前生命状态一致。0034 目标是把 Mood、Bond、Rest、Movement、DayPhase 等低敏上下文接入 DialogueContext，让 Demo Dialogue 和 BYOM 边界都能生成更贴近状态的短句。

## 当前产品阶段

v0.1.x Core Life。

## 产品范围

- DialogueContext 增加当前动画、行为、Mood、Bond、Rest、DayPhase 等低敏字段。
- Demo Dialogue 根据上下文选择短句。
- DialoguePolicy 继续限制长度、身份和语气。
- BYOM 默认仍不启用网络。

## 明确不做什么

- 不记录或发送用户长期语义记忆。
- 不把 Ian 做成聊天助手。
- 不默认联网。
- 不把内部数值暴露到 UI。

## 用户体验

Ian 刚睡醒、正在跑、熟悉用户或夜间时，短句会有轻微差异，但仍然短小、有角色感。

## 架构约束

- DialogueContext 由 Rust Core 组装。
- React 不拼接 prompt 或决定角色语气。
- Policy 对所有 provider 输出统一生效。

## 数据 / 协议变化

可扩展内部 DialogueContext，不一定进入前端协议。若进入协议，需要避免数值化 Bond / Mood 暴露。

## 隐私与安全边界

上下文只包含 Ian 自身低敏状态，不包含外部窗口、代码、终端或用户输入历史正文。

## 验收标准

- [ ] DialogueContext 包含核心生命状态摘要。
- [ ] Demo Dialogue 会根据至少 3 类上下文返回不同短句。
- [ ] Policy 仍会移除 AI assistant 身份并限制长度。
- [ ] BYOM 默认不联网、不需要 API key。
- [ ] 测试覆盖 mood、bond、sleep/day phase 和 policy。

## 验证方式

- Rust dialogue tests。
- Policy tests。
- Source scan 检查无敏感上下文进入 provider。
