# 0267 · Rare Idle Surprise Tuning

## 问题 / 目标

Idle 惊喜应该让 Ian 偶尔显得有自己的小生活，但不能频繁打扰。当前需要把触发频率、动作种类、时机和退出条件调清楚。

## 当前产品阶段

v0.1.x Life Feel / Idle Surprise。

## 产品范围

- 调整低频 idle surprise 的触发条件、冷却和预算。
- 明确几类可接受的 idle 小动作，例如伸懒腰、张望、轻跳、挥手、短暂停留。
- 支持诊断触发用于验收，正式体验保持低频。
- 记录 long-run 观察方式。

## 明确不做

- 不做复杂作息系统。
- 不做提醒或通知。
- 不做长时间大幅移动。
- 不读取用户工作内容来决定惊喜。

## 用户体验

用户偶尔看到 Ian 自己动一下，会觉得它在桌面上生活；但大多数时候它应保持安静，不抢注意力。

## 架构约束

- Idle surprise 由 Moment Orchestrator 和预算策略决定。
- React 不直接随机播放 idle 惊喜。
- DND、reduced motion、设置打开、气泡输入中必须抑制或降级。

## 数据 / 协议变化

尽量复用现有 animation / speech / effect action。新增动作必须保持语义化。

## 隐私与安全边界

只使用时间、Ian 状态和用户显式配置，不读取外部内容。

## 验收标准

- [ ] 诊断入口可以稳定触发每个 idle surprise 变体。
- [ ] 正式模式下 idle surprise 有全局预算和单项冷却。
- [ ] DND、reduced motion、设置打开、气泡输入中会抑制或降级。
- [ ] 单次 idle surprise 持续时间短且可被用户交互打断。
- [ ] 真实桌面 long-run 观察记录没有频繁打扰。

## 验证方式

- Rust 冷却 / 预算测试。
- 前端动画变体执行测试。
- 真实 Tauri 桌面诊断触发和长时间观察。
