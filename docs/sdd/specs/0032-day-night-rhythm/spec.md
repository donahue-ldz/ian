# Spec: Day Night Rhythm

## 状态

已实现，待验收。

## 问题 / 目标

Ian 的日常节奏需要有时间感。0032 目标是基于本地时间提供白天、傍晚、夜间的轻量行为差异，让移动、休息和短句更自然。

## 当前产品阶段

v0.1.x Core Life。

## 产品范围

- 定义本地 day phase：morning、day、evening、night。
- Scheduler / BehaviorPolicy 可根据 day phase 调整休息和移动频率。
- DialogueContext 可获得 day phase，但只用于短句语境。
- 用户后续可通过 0033 / 0038 控制安静时段。

## 明确不做什么

- 不读取日历。
- 不读取系统专注模式。
- 不联网获取天气、地点或日出日落。
- 不做健康提醒或作息评价。

## 用户体验

夜间 Ian 更安静、更容易休息；白天更愿意轻微活动。Ian 不评价用户作息。

## 架构约束

- Rust Core 基于本地时间计算 day phase。
- React 不决定 day phase。
- day phase 是低敏上下文，不应触发高频提示。

## 数据 / 协议变化

可新增内部 `DayPhase` 类型，必要时进入 `IanState` 或 `DialogueContext`。

## 隐私与安全边界

只使用本机时间和用户配置，不读取地理位置、日历或外部应用。

## 验收标准

- [ ] Rust Core 有可测试的 day phase 计算。
- [ ] night phase 会降低移动或提高休息概率。
- [ ] day phase 可进入 DialogueContext。
- [ ] 不读取日历、位置、天气或系统专注模式。
- [ ] 测试覆盖边界时间、模式差异和对话上下文。

## 验证方式

- Rust policy tests。
- Dialogue context tests。
- Source scan 检查无外部时间上下文读取。
