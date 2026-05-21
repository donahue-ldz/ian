# Spec: Basic Behavior

## 状态

已实现，已验证。

## 问题 / 目标

Ian 当前主要由点击和双击驱动，待机生命感较弱。0004 目标是在 P0 范围内补足基础行为：time tick 触发轻微状态变化，窗口内 mouse near 有反应，行为仍由 Rust Core 决策。

## 当前产品阶段

P0 / MVP + v0.1 Architecture Baseline。

## 产品范围

- 新增窗口内 `mouse.near` 事件。
- `BehaviorEngine` 根据 `time.tick`、`mouse.near`、`mouse.click` 输出基础动作。
- P0 行为限制在 idle、happy、walk、sleep、run。
- 前端只负责定时发送低频 tick 和窗口内 pointer enter/leave 事件。

## 明确不做什么

- 不做全局鼠标监听。
- 不读取窗口外鼠标位置。
- 不做完整 Mood/Bond。
- 不实现主动提醒。
- 不实现窗口感知或 app 感知。

## 用户体验

Ian 不操作时保持 idle 并偶尔微动；鼠标进入 Ian 窗口时会有轻微 happy/attention 反应；双击 run-around 到期仍回 idle。

## 架构约束

- 所有输入通过 `IanEvent`。
- 所有输出通过 `IanAction`。
- React 不决定行为，只发送窗口内事件和执行动作。

## 数据 / 协议变化

新增 `IanEvent::MouseNear { x, y }`，更新 TypeScript protocol placeholder。

## 隐私与安全边界

只处理 Ian 窗口内 pointer 事件，不使用系统级权限。

## 验收标准

- [ ] Rust protocol 包含 `mouse.near`。
- [ ] 前端 pointer enter 发送 `mouse.near`。
- [ ] Rust Core 对 `mouse.near` 返回 `animation.play happy` 或等价轻反应。
- [ ] `time.tick` 可返回 idle/walk/sleep 中的基础动作，且决策可测试。
- [ ] React 不直接决定行为。
- [ ] Playwright 可验证 pointer 进入后动画状态变化。

## 验证方式

- Rust behavior 单元测试。
- 前端测试。
- Playwright 本地验收。

