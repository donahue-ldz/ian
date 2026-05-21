# Spec: Keyboard Rhythm Opt In

## 状态

已实现，待验收。

## 问题 / 目标

键盘节奏可以帮助 Ian 理解用户是否在连续工作，但它属于高敏能力，不能默认开启。0019 目标是只做显式授权的键盘节奏信号：统计节奏强弱，不记录按键内容。

## 当前产品阶段

v0.2 Developer Rhythm。

## 产品范围

- 增加 KeyboardRhythmAdapter，默认关闭。
- 用户显式授权后，只记录节奏级别或事件计数。
- 不记录键值、文本、快捷键组合。
- 输出低敏 `keyboard.rhythm` 风格事件。

## 明确不做什么

- 不记录具体按键。
- 不记录输入文本。
- 不监听密码框内容。
- 不做键盘宏或快捷键。
- 不上传节奏数据。

## 用户体验

用户开启后，Ian 可以感知“你工作很久了”这类节奏，但不会知道用户具体输入了什么。

## 架构约束

- 高敏 adapter 默认关闭。
- Security Gate 必须检查显式授权。
- Adapter 输出只能是节奏摘要。
- React 不能直接访问键盘全局事件内容。

## 数据 / 协议变化

新增 keyboard rhythm summary event，payload 只包含时间窗口、强度、计数，不包含 key code 或文本。

## 隐私与安全边界

显式 opt-in。只记录统计摘要；关闭后必须停止采集。

## 验收标准

- [ ] KeyboardRhythmAdapter 默认关闭。
- [ ] 未授权时 Security Gate 拒绝 rhythm event。
- [ ] 授权后 payload 不包含 key、text、shortcut。
- [ ] 关闭授权后停止采集。
- [ ] 测试覆盖默认关闭、授权开启、payload 脱敏。

## 验证方式

- Rust security/adapter tests。
- source scan 检查无 key text 记录。
- 手动 opt-in/opt-out smoke。
