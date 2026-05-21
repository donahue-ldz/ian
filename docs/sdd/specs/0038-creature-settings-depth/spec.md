# Spec: Creature Settings Depth

## 状态

已实现，待验收。

## 问题 / 目标

核心生命感能力增加后，用户需要更细的控制，但设置不能变成复杂后台。0038 目标是扩展设置面，覆盖移动强度、安静程度、气泡频率、休息行为和资源包基础选项。

## 当前产品阶段

v0.1.x User Control。

## 产品范围

- 设置 movement intensity、bubble frequency、rest behavior、quiet hours。
- 设置结果持久化并进入 Rust Core policy。
- UI 保持紧凑，不做仪表盘。
- 默认值保持 P0/v0.1 轻量可用。

## 明确不做什么

- 不做复杂规则编辑器。
- 不展示内部 Mood / Bond 数值。
- 不做账号、云同步或远程配置。
- 不加入 Developer Rhythm 设置扩展。

## 用户体验

用户可以把 Ian 调得更安静或更活泼，而不需要理解内部系统。

## 架构约束

- 设置只写本地配置。
- Rust Core policy 消费配置。
- React 设置面不直接改变行为决策。

## 数据 / 协议变化

扩展 config schema。必要时扩展 `IanState` 返回当前设置摘要，但不暴露内部数值。

## 隐私与安全边界

只保存用户显式配置，不采集行为外部数据。

## 验收标准

- [ ] 设置面可调整移动强度、气泡频率、休息行为和安静时段。
- [ ] 设置持久化并重启恢复。
- [ ] Rust Core policy 使用这些设置。
- [ ] UI 不展示 Mood / Bond 数值。
- [ ] 测试覆盖设置读写、默认值、策略消费和 UI 状态。

## 验证方式

- Rust config tests。
- Behavior policy tests。
- Frontend settings tests。
- Browser smoke：设置修改后 UI 状态稳定。
