# Spec: Run Around Path Behavior

## 状态

已实现，待验收。

## 问题 / 目标

当前双击 run-around 主要表现为 run 动画。0022 目标是把双击彩蛋升级为短路径真实跑动：Ian 会跑几个安全点，然后回到 idle 或停在合理位置。

## 当前产品阶段

v0.1.x Core Life。

## 产品范围

- 双击 Ian 后 Rust Core 生成 run-around 动作序列。
- 动作序列包含 run 动画和 2-4 个 `movement.move_to` waypoint。
- 路径短、可预测、可测试。
- run-around 到期后回到 idle。

## 明确不做什么

- 不做全屏复杂寻路。
- 不做主动开发者庆祝。
- 不读取外部窗口位置。
- 不做物理碰撞、障碍物或多宠物路径。

## 用户体验

用户双击 Ian 后，Ian 会短暂兴奋地跑一小圈，而不是只在原地播放 run 动画。

## 架构约束

- 路径由 Rust Core / BehaviorPolicy 决定。
- React 只顺序执行 `IanAction`。
- 路径必须受 0021 执行链路和 0024 边界策略约束。

## 数据 / 协议变化

优先复用 `BehaviorRunAround` 和 `MovementMoveTo`。如现有单 action 无法表达路径，可新增内部 action sequence 执行逻辑，但 Rust 仍是路径来源。

## 隐私与安全边界

只使用 Ian 自身位置、配置和安全边界，不读取外部 App 或屏幕内容。

## 验收标准

- [ ] 双击事件进入 Rust Core 后返回 run-around 相关 action。
- [ ] run-around 包含至少 2 个真实 movement waypoint。
- [ ] waypoint 不由 React 临时生成。
- [ ] run-around 结束后动画回到 idle。
- [ ] 自动测试覆盖路径数量、动作顺序和结束状态。

## 验证方式

- Rust behavior 单元测试。
- 前端 action sequence 测试。
- Playwright / Tauri smoke：双击后 Ian 位置发生变化并回 idle。
