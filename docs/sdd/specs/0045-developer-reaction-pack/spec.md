# Spec: Developer Reaction Pack

## 状态

已实现，待验收。

## 问题 / 目标

Git 和 build/test 事件需要统一的 Ian 风格反应，否则会退化成通知器。0045 目标是建立 Developer Reaction Pack：一组短句、动画、移动和冷却规则，专门服务开发者节奏陪伴。

## 当前产品阶段

v0.2 Developer Rhythm。

## 产品范围

- 定义成功、失败、连续失败、清理完成、节奏过密等反应。
- 所有文案短、克制、角色化。
- 反应必须有 cooldown 和 burst 限制。
- 反应可被 quiet hours / snooze / busy policy 降级。

## 明确不做什么

- 不输出代码建议。
- 不解释错误堆栈。
- 不催促用户提交或修测试。
- 不做任务列表。

## 用户体验

Ian 像陪在旁边的小生命：测试过了会开心一下，连续失败会温和陪伴，但不会指挥用户。

## 架构约束

- Reaction Pack 位于 Rust Core policy / dialogue 边界。
- React 只执行 action。
- DialoguePolicy 继续限制长度和身份。

## 数据 / 协议变化

可新增内部 `DeveloperReactionKind`，不必暴露到前端协议。

## 隐私与安全边界

反应只能基于已通过 Security Gate 的低敏摘要。

## 验收标准

- [ ] 成功、失败、连续失败至少各有一种 Ian 风格反应。
- [ ] 反应有 cooldown 和 burst 限制。
- [ ] quiet hours / snooze 可降级或关闭反应。
- [ ] 文案不包含 AI assistant 身份或生产力训诫。
- [ ] 测试覆盖 reaction kind、cooldown、burst、policy 降级。

## 验证方式

- Rust policy tests。
- Dialogue policy tests。
- Browser smoke：连续 mock event 不刷屏。
