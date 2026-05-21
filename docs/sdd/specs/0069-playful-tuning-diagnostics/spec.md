# Spec: Playful Tuning Diagnostics

## 状态

已实现，待用户验收。

## 问题 / 目标

高能随机行为如果无法解释，会很难调参和验收。0069 目标是建立本地低敏诊断，记录 playful 行为为什么触发、为什么被冷却或为什么被安全网拦截。

## 产品范围

- 记录 trigger reason、cooldown result、safety gate result、chosen reaction key。
- 诊断只保留本地低敏摘要。
- 提供开发期查看方式，可以是日志、诊断面板或测试输出。
- 支持调参：频率、冷却、路径长度、反应权重。

## 明确不做什么

- 不上传诊断。
- 不记录用户文本、代码、窗口标题或键盘内容。
- 不把诊断作为普通用户主路径。
- 不做复杂 A/B 实验平台。

## 用户体验

开发者或验收者能看懂 Ian 为什么刚才没乱跑、为什么说了某句、为什么进入冷却，从而调到“可爱但不烦”的强度。

## 架构约束

- 诊断由 Rust Core 输出低敏摘要。
- React 可以展示诊断，但不能用诊断结果反向决定行为。
- 诊断必须遵守本地优先和隐私边界。

## 数据 / 协议变化

可新增 local diagnostic event 或复用 0039 的 core life telemetry。字段必须是 reason / key / timestamp / result，不含敏感内容。

## 隐私与安全边界

诊断内容禁止包含用户输入、应用标题、URL、代码、终端全文、文件路径。

## 验收标准

- [x] playful 触发和拒绝都有低敏 reason。
- [x] 诊断能区分 cooldown、quiet、off、active interaction 等拦截原因。
- [x] 诊断不含敏感内容。
- [x] 调参值可在本地配置或测试 fixture 中调整。
- [x] 测试覆盖诊断字段。

## 验证方式

- Rust telemetry / diagnostics tests。
- 隐私字段审查。
- 本地 smoke：触发和被拒绝时能看到 reason。
