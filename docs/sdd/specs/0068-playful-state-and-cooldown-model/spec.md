# Spec: Playful State And Cooldown Model

## 状态

草稿，待确认。

## 问题 / 目标

高能行为需要状态和冷却模型，否则会重复触发、互相抢占或无法解释。0068 目标是建立 playful state、疲劳、恢复和冷却模型。

## 产品范围

- 定义 playful state：idle、warming_up、zooming、settling、cooling_down。
- 定义 cooldown key：zoomies、cute_phrase、affection_burst、idle_surprise。
- 高能后进入 cooling_down，短时间内减少再次触发。
- 用户互动可以轻微恢复 playful 能量，但不能无限叠加。

## 明确不做什么

- 不做复杂数值养成。
- 不展示能量条或疲劳值。
- 不把 cooldown 写散在 React 组件里。
- 不让多个高能 action 同时运行。

## 用户体验

Ian 高能后会喘口气或安静一下，不会连续疯狂跑；用户继续互动时，它可以慢慢恢复兴奋。

## 架构约束

- 状态和冷却归 Rust Core。
- UI 只接收当前行为或动作，不显示内部数值。
- 冷却模型必须可测试并可被诊断摘要解释。

## 数据 / 协议变化

可新增内部 `PlayfulState`。如需要同步到前端，只暴露粗粒度状态，不暴露数值。

## 隐私与安全边界

只记录 Ian 自身行为摘要和本地时间，不记录用户内容。

## 验收标准

- [ ] zoomies 运行时不会再启动第二个 zoomies。
- [ ] 高能结束后进入冷却，冷却期内触发被降级或拒绝。
- [ ] 冷却不会阻止普通点击反馈。
- [ ] 内部状态不以数值面板展示给用户。
- [ ] 测试覆盖状态切换、冷却和恢复。

## 验证方式

- Rust state / behavior tests。
- 本地诊断摘要检查。
- Browser smoke：连续触发不会叠加失控。
