# Spec: Life Rhythm Composition

## 状态

已实现，待验收。

## 问题 / 目标

0021-0029 会分别建立移动、跑动、游走、休息、好奇、亲近和回窝能力。0030 目标是做一次组合收束：建立统一 Life Rhythm Policy，避免多个能力互相抢占，让 Ian 的日常节奏稳定、克制、可控。

## 当前产品阶段

v0.1.x Core Life。

## 产品范围

- 统一调度 Movement、Rest、Attention、Affection 的优先级。
- 明确用户输入优先级高于自主行为。
- 行为模式统一影响频率、幅度和打扰程度。
- 增加组合级回归测试。

## 明确不做什么

- 不新增 Developer Rhythm 功能。
- 不做复杂长期记忆。
- 不做任务系统、提醒中心或生产力仪表盘。
- 不读取外部上下文。

## 用户体验

Ian 会自然地待机、走动、休息、回应用户、回到自己的位置；这些行为不会互相打架，也不会让用户觉得被打扰。

## 架构约束

- Life Rhythm Policy 位于 Rust Core。
- Adapter 仍只提供事件。
- React 只执行 action，不做优先级判断。
- 组合策略必须可测试、可降级。

## 数据 / 协议变化

可新增内部 policy state，例如 cooldown、last_action、current_life_phase。不新增用户可见复杂状态，除非后续 SDD 单独批准。

## 隐私与安全边界

只组合 Ian 自身状态、本地 tick、本地互动和用户配置。

## 验收标准

- [ ] 用户输入优先于自主 roam / sleep。
- [ ] run-around 不被 sleep / roam 抢占。
- [ ] behavior mode 统一影响移动、休息和好奇反应。
- [ ] 多个候选动作同时出现时只有一个明确胜出策略。
- [ ] 组合测试覆盖 idle、run、sleep、click、drag、anchor 回归。

## 验证方式

- Rust policy / behavior integration tests。
- Frontend regression tests。
- 长时 smoke：无 action 冲突、无 console error、Ian 可被用户拖回。
