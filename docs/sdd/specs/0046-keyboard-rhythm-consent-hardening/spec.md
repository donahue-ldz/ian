# Spec: Keyboard Rhythm Consent Hardening

## 状态

已实现，待验收。

## 问题 / 目标

0019 已经建立键盘节奏 opt-in skeleton。0046 目标是把它加固到可产品化：用户明确授权、可随时停止、payload 保证不含 key/text/shortcut，并能证明关闭后不再采集。

## 当前产品阶段

v0.2 Developer Rhythm。

## 产品范围

- 键盘节奏单独授权，不跟随 Developer Rhythm 总开关自动开启。
- 只记录时间窗口、强度、计数。
- 关闭后清空未发送队列或缓存。
- 设置面解释“不记录按键内容”。

## 明确不做什么

- 不记录 key code。
- 不记录输入文本。
- 不记录快捷键组合。
- 不监听密码框内容。
- 不上传节奏数据。

## 用户体验

用户可以选择让 Ian 感知“你工作很久了”这种节奏，但 Ian 永远不知道用户具体输入了什么。

## 架构约束

- Adapter 默认关闭。
- Security Gate 检查显式授权。
- Sanitizer 拒绝任何 key/text/shortcut 字段。
- React 不直接接触全局键盘内容。

## 数据 / 协议变化

复用 `keyboard.rhythm`，payload 只包含 `window_ms`、`intensity`、`count`。

## 隐私与安全边界

高敏 opt-in。关闭后必须停止采集并清理队列。

## 验收标准

- [ ] Keyboard rhythm 默认关闭，且不被 Developer Rhythm 总开关自动开启。
- [ ] 授权后 payload 只包含 window、intensity、count。
- [ ] sanitizer 拒绝 key、text、shortcut 等字段。
- [ ] 关闭授权后 adapter 停止采集并清空队列。
- [ ] 测试覆盖默认关闭、授权开启、关闭清理、payload 脱敏。

## 验证方式

- Rust adapter / security / sanitizer tests。
- Source scan 检查无 key/text 记录。
- 手动 opt-in / opt-out smoke。
