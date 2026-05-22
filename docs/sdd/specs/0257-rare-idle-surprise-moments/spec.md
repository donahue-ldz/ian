# 0257 · Rare Idle Surprise Moments

## 问题 / 目标

Ian 的 idle 如果只有循环动画，会缺少“它自己在生活”的感觉。本 SDD 增加低频 idle 惊喜：整理装备、探头、伸懒腰、打瞌睡等短瞬间。

## 当前产品阶段

v0.1.x / Life Feel。

## 产品范围

- 定义低频 idle surprise moment 池。
- 每天 / 每小时有预算，不频繁触发。
- 只在用户未交互、未勿扰、未设置打开时触发。
- 资源包缺动作时回退到现有动画。

## 明确不做

- 不做持续表演。
- 不做声音。
- 不要求用户回应。

## 用户体验

用户偶尔瞥见 Ian 自己整理装备、探头或犯困，会觉得它在桌面上生活，但不会频繁打扰。

## 架构约束

- Rust Core 决定低频触发。
- Moment 受 budget 和 cooldown 控制。
- React 只播放动画和短 effect。

## 数据 / 协议变化

可新增低敏 moment budget 状态；优先内存态，避免不必要持久化。

## 隐私与安全边界

Idle surprise 不依赖外部敏感输入，不读取用户内容。

## 验收标准

- [ ] idle surprise 低频触发。
- [ ] 用户交互、拖动、输入、设置打开时不触发。
- [ ] 勿扰或 reduced motion 下减少或关闭。
- [ ] 缺失动画时安全 fallback。
- [ ] 桌面验收能观察到至少一种 rare idle moment。

## 验证方式

- Rust 时间和 budget 测试。
- 资源包 fallback 测试。
- 真实桌面长一点的 idle 验收。
