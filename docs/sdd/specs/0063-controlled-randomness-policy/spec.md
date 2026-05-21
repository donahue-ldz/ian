# Spec: Controlled Randomness Policy

## 状态

已实现，待用户验收。

## 问题 / 目标

Ian 需要更随机、更像有主意，但测试和验收不能变得不稳定。0063 目标是建立受控随机策略：线上表现有变化，测试中可复现。

## 产品范围

- 引入 seedable randomness 或 deterministic random provider。
- 随机用于 playful trigger、路径候选、短句选择和动作组合。
- 随机结果受行为模式、冷却、安静模式和状态优先级约束。
- 测试可固定 seed 验证输出。

## 明确不做什么

- 不使用不可复现随机直接决定核心行为。
- 不让 React 使用 `Math.random` 决定 Ian 行为。
- 不用随机绕过安全边界。
- 不让随机影响隐私或权限判断。

## 用户体验

Ian 的高能彩蛋和撒娇反馈不会每次完全一样，但也不会突然失控或频繁重复。

## 架构约束

- Random provider 属于 Rust Core policy 层。
- 所有随机候选最终仍输出明确 `IanAction`。
- 测试必须能注入固定 seed 或固定随机序列。

## 数据 / 协议变化

通常不需要用户可见协议。可在内部 policy 中新增 random seed / random source 抽象。

## 隐私与安全边界

随机源不能来自用户敏感内容、键盘输入、窗口标题、代码内容或网络。

## 验收标准

- [x] 测试中可以固定随机输出。
- [x] React 不负责行为随机决策。
- [x] 随机候选经过冷却和边界策略。
- [x] 同类短句和路径不会机械重复。
- [x] 随机不会影响权限和隐私 gate。

## 验证方式

- Rust unit tests 覆盖固定 seed。
- Behavior integration tests 覆盖随机候选被 policy 约束。
- Code review 检查 React 无行为随机源。
