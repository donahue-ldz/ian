# Spec: Affection Touch Loop

## 状态

已实现，待验收。

## 问题 / 目标

Ian 的亲近感需要来自反复但克制的本地互动，而不是等级系统。0028 目标是把点击、轻抚、连续短互动转为更自然的亲近反馈：表情、短句、靠近或小幅开心动作。

## 当前产品阶段

v0.1.x Core Life。

## 产品范围

- 区分单击、短时间连续点击、拖拽结束后的轻反馈。
- BondState 可影响反馈语气和表情，但不展示数值。
- 反馈有冷却，避免连续点击刷屏。
- 支持“被打扰太多会短暂躲开”的轻反应。

## 明确不做什么

- 不展示等级、经验条、亲密度数字。
- 不做成就系统。
- 不做复杂手势识别。
- 不读取触控板或系统级手势。

## 用户体验

用户轻点或轻抚 Ian 时，它会给出短小、亲近、可爱的反应；连续打扰时会有一点点躲避，但不惩罚用户。

## 架构约束

- Rust Core 根据 interaction event 和 BondState 决定反馈。
- React 只发送已有窗口内事件并执行 action。
- DialoguePolicy 继续控制短句长度和身份。

## 数据 / 协议变化

可复用 `MouseClick`、`MouseDragStart`、`MouseDragEnd` 和 BondState。必要时新增轻量 interaction kind，但不新增复杂手势模型。

## 隐私与安全边界

只使用 Ian 窗口内互动事件和本地 BondState。

## 验收标准

- [ ] 连续本地互动可产生不同于单次点击的轻反馈。
- [ ] BondState 影响亲近反馈，但 UI 不展示数值。
- [ ] 连续点击有 cooldown 或降频。
- [ ] 过度打扰可触发小幅躲开或短句。
- [ ] 测试覆盖单击、连续点击、cooldown、Bond context。

## 验证方式

- Rust bond / behavior tests。
- Dialogue policy tests。
- Browser smoke：连续点击不会刷屏，且 UI 无数值亲密度。
