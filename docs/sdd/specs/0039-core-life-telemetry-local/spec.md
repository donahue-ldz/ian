# Spec: Core Life Telemetry Local

## 状态

已实现，待验收。

## 问题 / 目标

核心生命感行为变多后，需要可诊断性，否则很难解释 Ian 为什么移动、睡觉或不回应。0039 目标是建立本地诊断视图或日志，记录行为决策摘要，默认不上传、不含敏感内容。

## 当前产品阶段

v0.1.x Diagnostics。

## 产品范围

- 记录 behavior decision summary：输入事件类型、候选动作、最终动作、cooldown 原因。
- 提供本地 debug 读取方式，默认面向开发/验收。
- 对敏感字段做 allowlist。
- 可在 verification 中帮助定位行为冲突。

## 明确不做什么

- 不上传 telemetry。
- 不记录用户输入正文。
- 不记录外部窗口标题、URL、代码、终端。
- 不做产品分析埋点平台。

## 用户体验

普通用户无需看到诊断；开发和验收时可以理解 Ian 的行为来源，降低“跑偏”风险。

## 架构约束

- Rust Core 生成诊断摘要。
- 诊断写入本地、可关闭、低敏。
- React 若展示，只展示受控摘要。

## 数据 / 协议变化

可新增 local diagnostic repository 或复用 life event journal 的 diagnostic event 类型。

## 隐私与安全边界

默认本地，不上传。只允许事件类型、动作类型、策略原因等低敏字段。

## 验收标准

- [ ] 行为决策摘要可本地记录。
- [ ] 摘要不包含用户输入正文或外部上下文。
- [ ] 可关闭或限制保留期。
- [ ] 至少一个行为冲突测试能通过诊断原因解释。
- [ ] Source scan 覆盖敏感字段未写入。

## 验证方式

- Rust diagnostics tests。
- Source scan。
- 本地 smoke：触发 roam / sleep / click 后能看到低敏摘要。
