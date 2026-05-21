# Spec: Simple Mood State

## 状态

已实现，已验证。

## 问题 / 目标

Ian 需要逐步表现出情绪连续性，但 P0 不应实现完整 Mood System。0010 目标是建立简单 Mood State：只基于 Ian 内部轻量事件调整状态，并影响语气和少量动画选择。

## 当前产品阶段

v0.1.x Product Iteration。

## 产品范围

- Rust Core 定义简单 `MoodState`。
- 支持 calm、happy、sleepy 等少量状态。
- 点击、对话、低频 tick 可产生 MoodSignal。
- Mood 影响 Demo Dialogue 语气和部分行为候选。

## 明确不做什么

- 不做复杂情绪模型。
- 不读取用户外部行为。
- 不展示情绪数值、等级或仪表盘。
- 不把 Mood 作为完整长期画像。

## 用户体验

Ian 的回复和小动作会有轻微状态连续性，例如刚互动后更活泼，久未互动后更安静。

## 架构约束

- MoodEngine 属于 Rust Core。
- Mood 只输出状态或影响 policy，不直接操控 React。
- React 不计算 Mood。

## 数据 / 协议变化

可在 `IanState` 中暴露简化 mood view，也可仅内部使用；若暴露，TypeScript 类型必须同步。

## 隐私与安全边界

Mood 只基于 Ian 窗口内互动和本地时间，不使用外部敏感数据。

## 验收标准

- [ ] Rust Core 存在简单 `MoodState` 和 `MoodSignal`。
- [ ] 点击或对话可更新 mood，且有测试覆盖。
- [ ] Demo Dialogue 可接收 mood context 并改变短句风格。
- [ ] Mood 不在 UI 中显示数值或等级。
- [ ] React 不直接计算 mood。

## 验证方式

- Rust mood 单元测试。
- Dialogue policy/context 测试。
- 前端 typecheck。
