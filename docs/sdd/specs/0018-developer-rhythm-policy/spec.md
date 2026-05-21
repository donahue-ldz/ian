# Spec: Developer Rhythm Policy

## 状态

已实现，待验收。

## 问题 / 目标

0016 和 0017 会引入开发节奏事件，但如果直接把每个事件映射成动画，Ian 会变成开发工具通知器。0018 目标是建立 Developer Rhythm Policy，统一决定哪些开发事件值得回应、回应强度和冷却时间。

## 当前产品阶段

v0.2 Developer Rhythm。

## 产品范围

- Rust Core 增加 DeveloperRhythmPolicy。
- 聚合 Git/build/test 等低敏事件。
- 控制回应频率、冷却时间和打扰等级。
- 只输出 Ian 风格的短动作或短气泡。

## 明确不做什么

- 不实现新的 adapter。
- 不读取代码或终端。
- 不做任务管理通知中心。
- 不把每个事件都提示给用户。

## 用户体验

Ian 会偶尔对开发节奏产生有生命感的反应，例如测试连续失败时短暂担心，提交成功时开心一下，但不会刷屏。

## 架构约束

- Policy 位于 Rust Core。
- Adapter 只提供事件，不决定行为。
- React 只执行 `IanAction`。
- Policy 必须可测试、可配置默认频率。

## 数据 / 协议变化

可新增内部 developer rhythm state，不要求新增用户可见状态。

## 隐私与安全边界

Policy 只消费已通过 Security Gate 的低敏事件，不接触原始代码或日志。

## 验收标准

- [ ] DeveloperRhythmPolicy 可根据事件类型和频率决定是否回应。
- [ ] 同类事件有冷却时间，避免刷屏。
- [ ] 连续 failure 可产生不同于单次 failure 的轻反应。
- [ ] Policy 不读取 adapter 原始敏感内容。
- [ ] Rust policy 测试覆盖 success、failure、cooldown、burst。

## 验证方式

- Rust policy 单元测试。
- BehaviorEngine 集成测试。
- Playwright smoke 确认 UI 不被连续事件刷屏。
