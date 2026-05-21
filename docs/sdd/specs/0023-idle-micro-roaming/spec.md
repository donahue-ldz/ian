# Spec: Idle Micro Roaming

## 状态

已实现，待验收。

## 问题 / 目标

Ian 需要在无人操作时也有轻微存在感。0023 目标是让 Ian 在 idle 状态下低频、小范围、自主走动，但不能打扰用户或造成失控感。

## 当前产品阶段

v0.1.x Core Life。

## 产品范围

- Scheduler 在合适 tick 上产生低频 roam 候选。
- BehaviorMode 影响频率：quiet 极少，normal 少量，lively 稍多。
- 每次移动幅度有限，优先围绕当前位置或 home 附近。
- 用户拖拽、run-around、bubble 输入期间不抢占。

## 明确不做什么

- 不做全屏乱跑。
- 不做基于工作状态的移动。
- 不读取键盘、窗口、Git 或应用类别。
- 不做提醒文案。

## 用户体验

Ian 偶尔走两步、停一下，像有自己的小动作；安静模式下几乎不主动移动。

## 架构约束

- 自主移动由 Rust Core scheduler / behavior policy 决定。
- React 只执行 movement action。
- 自主移动必须经过边界策略。

## 数据 / 协议变化

复用 `TimeTick` 和 `MovementMoveTo`。可新增内部 cooldown state，不新增用户可见状态。

## 隐私与安全边界

只使用本地时间 tick、Ian 当前状态和用户配置。

## 验收标准

- [ ] `time.tick` 可在非 quiet 模式下低频产生 roam movement。
- [ ] quiet 模式显著减少或关闭自主移动。
- [ ] run-around、拖拽、输入期间不会被 roam 抢占。
- [ ] roam 目标由 Rust Core 产生。
- [ ] 测试覆盖频率、模式差异和抢占保护。

## 验证方式

- Rust scheduler / behavior tests。
- 前端回归测试。
- 长一点的本地 smoke：观察 idle 状态下低频移动且无 console error。
