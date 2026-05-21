# Spec: Playful Safety And User Control

## 状态

已实现，待用户验收。

## 问题 / 目标

高能行为必须有明确安全网。0067 目标是确保用户能停止、降低或关闭满屏乱跑和撒娇卖萌，并且 Ian 不会在重要场景中打扰。

## 产品范围

- 用户可关闭 playful spontaneous behavior。
- 提供“暂停高能一段时间”或复用 snooze / quiet mode。
- 任意用户主动拖拽、输入、打开设置都可取消或阻止高能行为。
- 高能行为必须有最长持续时长。

## 明确不做什么

- 不做强制陪伴。
- 不隐藏关闭入口。
- 不用高能行为覆盖用户操作。
- 不读取外部忙碌状态，除非已有授权 category。

## 用户体验

用户喜欢时能让 Ian 更活泼，不想被打扰时能立刻让它安静下来。

## 架构约束

- Safety gate 在 Rust Core policy 层统一判断。
- React 的取消按钮或设置变更必须转成 event / command。
- 高能 action executor 必须支持取消或过期。

## 数据 / 协议变化

可新增 `playful_snoozed_until` 或复用现有 snooze。可新增 cancel event，例如 `behavior.cancel_playful`。

## 隐私与安全边界

不新增敏感数据读取。若使用 active app category，必须遵守已授权、粗粒度、无标题和无 URL 的边界。

## 验收标准

- [x] 用户能关闭自发高能行为。
- [x] 高能行为有最长持续时长。
- [x] 拖拽、输入、设置打开会阻止或取消高能行为。
- [x] quiet mode / quiet hours 下不会自发触发。
- [x] 测试覆盖关闭、暂停、取消和过期。

## 验证方式

- Rust policy tests。
- 前端设置和取消测试。
- Browser smoke：触发后取消、关闭后不触发。
