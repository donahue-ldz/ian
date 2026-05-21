# Spec: Rest And Sleep Cycle

## 状态

已实现，待验收。

## 问题 / 目标

Ian 需要有安静的休息节奏，而不是一直待机或一直动。0026 目标是建立轻量休息 / 睡眠循环：Ian 可以趴下、短睡、醒来，并受行为模式、用户互动和移动状态约束。

## 当前产品阶段

v0.1.x Core Life。

## 产品范围

- Scheduler 可在低频 tick 上触发 rest / sleep 候选。
- 用户互动可以温和唤醒 Ian。
- quiet 模式更容易休息，lively 模式更少长时间睡。
- sleep 不抢占 run-around、拖拽、bubble 输入。

## 明确不做什么

- 不做完整 Sleep System。
- 不基于用户键盘、窗口或工作状态判断疲劳。
- 不做闹钟、提醒或日程。
- 不展示睡眠数值或健康指标。

## 用户体验

Ian 偶尔趴下或打盹；用户点击时会醒来或给出轻微反应。整体感觉像一个会休息的小生命，而不是通知工具。

## 架构约束

- 休息状态由 Rust Core 决定。
- React 只播放 sleep / wake 相关动作。
- Mood / Bond 可以作为上下文影响概率，但不直接在 React 决策。

## 数据 / 协议变化

优先复用 `AnimationPlay sleep`、`AnimationPlay idle` 和现有 `IanState.current_behavior`。如需新增 rest 状态，应从 Rust 协议生成到 TypeScript。

## 隐私与安全边界

只使用本地 tick、Ian 自身状态、行为模式和本地互动事件。

## 验收标准

- [ ] Rust Core 可在允许条件下输出 sleep/rest action。
- [ ] 用户 click / near 可让 Ian 从 sleep 回到 idle/happy。
- [ ] run-around、拖拽、输入期间不会被 sleep 抢占。
- [ ] BehaviorMode 影响 rest 频率。
- [ ] 测试覆盖休息触发、唤醒、抢占保护和模式差异。

## 验证方式

- Rust scheduler / behavior tests。
- 前端 animation state tests。
- Browser smoke：sleep、wake 动画切换无空白帧。
