# Spec: Behavior Scheduler

## 状态

已实现，已验证。

## 问题 / 目标

0004 已有基础行为，但行为仍主要由即时事件触发。0009 目标是建立轻量调度器，让 Ian 在不打扰的前提下拥有更自然的 idle variation、短暂休息和低频自发动作。

## 当前产品阶段

v0.1.x Product Iteration。

## 产品范围

- Rust Core 内增加轻量 behavior scheduler。
- `time.tick` 进入 scheduler，输出低频 `IanAction`。
- 行为受行为模式约束：安静更少、活泼稍多。
- 输出仍限制在 idle/walk/sleep/happy 等 P0/v0.1.x 允许动画。

## 明确不做什么

- 不做主动提醒文案。
- 不读取用户工作状态。
- 不接入全局键盘、Git、窗口感知。
- 不做复杂 Mood/Bond 驱动。

## 用户体验

Ian 不被点击时也会偶尔动一下或短暂打盹，但频率低、可控、不打扰。

## 架构约束

- Rust Core 决定何时触发行为。
- React 只发送 tick 和执行 action。
- Scheduler 必须可测试，避免随机导致测试不稳定。

## 数据 / 协议变化

可增加内部 scheduler state；不新增外部用户可见协议。

## 隐私与安全边界

只使用本地时间 tick 和 Ian 自身状态，不读取外部环境。

## 验收标准

- [ ] `time.tick` 经过 Rust scheduler 产生可测试的 idle variation。
- [ ] 行为模式会影响调度频率或候选动作。
- [ ] Scheduler 不在 run-around 或用户输入期间抢占当前动作。
- [ ] React 不直接决定自发行为。
- [ ] Rust scheduler 测试通过。

## 验证方式

- Rust behavior scheduler 单元测试。
- 前端回归测试。
- Playwright 长一点的本地 smoke，确认无 console error。
