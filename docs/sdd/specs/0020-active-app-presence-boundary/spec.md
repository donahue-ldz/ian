# Spec: Active App Presence Boundary

## 状态

已实现，待验收。

## 问题 / 目标

Ian 后续可能根据用户所在应用调整存在感，但活动应用信息也可能暴露隐私。0020 目标是建立 active app presence 的安全边界：只允许显式授权后的粗粒度类别，不读取窗口标题、文档名或屏幕内容。

## 当前产品阶段

v0.2 Developer Rhythm。

## 产品范围

- 定义 active app presence adapter skeleton。
- 默认关闭，必须显式授权。
- 只输出 app category 或允许列表中的 bundle id。
- 不输出窗口标题、文件名、URL、屏幕文字。

## 明确不做什么

- 不做 window-aware behavior。
- 不读取窗口标题。
- 不读取屏幕 OCR。
- 不读取浏览器 URL。
- 不做跨应用自动化。

## 用户体验

启用后，Ian 最多知道用户大概在“编辑器 / 浏览器 / 会议”类别中，从而调整打扰程度；不显示或存储具体窗口内容。

## 架构约束

- Adapter 输出必须先经过 Security Gate。
- BehaviorPolicy 只能消费粗粒度 presence。
- React 不访问系统窗口信息。

## 数据 / 协议变化

新增 active app presence summary event，payload 只包含 category、timestamp、confidence 或 allowlisted app id。

## 隐私与安全边界

默认关闭。不读取窗口标题、URL、文档名、屏幕内容。

## 验收标准

- [ ] Active app presence adapter skeleton 默认关闭。
- [ ] 未授权事件被 Security Gate 拒绝。
- [ ] payload 不包含窗口标题、URL、文件名或屏幕文字。
- [ ] BehaviorPolicy 可基于 app category 降低提醒打扰。
- [ ] 测试覆盖默认关闭、payload 脱敏、policy 使用 category。

## 验证方式

- Rust adapter/security/policy tests。
- source scan 检查无 title/OCR/URL 采集。
- 手动授权 smoke。
