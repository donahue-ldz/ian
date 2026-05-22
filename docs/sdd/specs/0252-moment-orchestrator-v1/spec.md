# 0252 · Moment Orchestrator V1

## 问题 / 目标

Ian 已经有动画、气泡、移动、找回、睡眠、记忆等点状能力，但缺少把这些能力组合成“有铺垫的小瞬间”的编排层。本 SDD 建立 Moment Orchestrator，让 Ian 的惊喜感来自短场景，而不是随机堆动画。

## 当前产品阶段

v0.1.x / Life Feel。

## 产品范围

- 定义 `IanMoment` 或等价内部模型。
- 建立 moment 选择、冷却、互斥和优先级规则。
- 第一版只编排已有 `IanAction`，不新增复杂视觉资产。
- Moment 由 Rust Core 决定，React 只执行动作。

## 明确不做

- 不做任务提醒器。
- 不做大段聊天。
- 不做社交、插件或联网能力。
- 不让 React 随机决定 Ian 行为。

## 用户体验

用户会感觉 Ian 偶尔有“自己的小动作”：被叫回时跑出来、鼠标靠近时好奇、放下后安顿一下。这些瞬间短、低频、可打断，不干扰工作。

## 架构约束

- Rust Core 继续作为行为大脑，Moment Orchestrator 属于 Rust Core / BehaviorPolicy。
- Moment 输入来自 `IanEvent` 和 `IanState`，输出仍是 `IanAction`。
- React 不保存 moment 状态，只执行动作和渲染效果。
- 所有 moment 必须受冷却、用户交互状态、勿扰和 reduced motion 约束。

## 数据 / 协议变化

优先复用现有 `IanAction`。如需要新增 `moment.started` 诊断动作或状态字段，必须只用于低敏调试，不暴露复杂内部数值。

## 隐私与安全边界

- Moment 不读取代码、屏幕、剪贴板、私聊、全局键盘文本或终端输出。
- Moment 不上传数据。
- 记忆相关 moment 只能使用用户确认过的低敏 tags。

## 验收标准

- [ ] Rust Core 中存在清晰的 moment 编排边界。
- [ ] Moment 输出仅由 `IanAction` 表达。
- [ ] Moment 有优先级、冷却和互斥规则。
- [ ] Moment 不会在拖动、输入气泡或设置打开时抢占。
- [ ] 测试覆盖至少 3 类 moment 的选择和冷却。

## 验证方式

- 运行 Rust 目标测试。
- 运行前端类型检查，确认协议一致。
- 涉及桌面可见行为的后续 moment 必须真实 Tauri 桌面验收。
