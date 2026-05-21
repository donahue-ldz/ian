# Spec: Active App Disturbance Policy

## 状态

已实现，待验收。

## 问题 / 目标

0020 已建立 active app presence boundary。0047 目标是只使用粗粒度 app category 来降低 Ian 打扰程度，例如会议、演示、专注时减少移动和气泡，不做窗口感知。

## 当前产品阶段

v0.2 Developer Rhythm。

## 产品范围

- 使用 category：editor、browser、meeting、presentation、focus、unknown。
- meeting / presentation / focus 降低主动行为。
- active app presence 默认关闭，显式授权后才生效。
- Policy 只消费 category，不消费标题或 URL。

## 明确不做什么

- 不读取窗口标题。
- 不读取浏览器 URL。
- 不读取文档名。
- 不做屏幕 OCR。
- 不做跨应用自动化。

## 用户体验

用户开会或演示时，Ian 会更安静，少动少说；回到普通编辑或浏览时恢复正常节奏。

## 架构约束

- Adapter 输出 category summary。
- Security Gate 校验授权和 payload 脱敏。
- BehaviorPolicy / ReminderPolicy 决定降打扰。

## 数据 / 协议变化

复用 `active_app.presence`。payload 只包含 category、confidence、可选 allowlisted app id。

## 隐私与安全边界

不允许窗口标题、URL、文件名、屏幕文字进入 payload。

## 验收标准

- [ ] 未授权时 active app presence 被拒绝。
- [ ] meeting / presentation / focus 会降低主动移动、气泡或提醒。
- [ ] policy 只读取 category。
- [ ] payload 不含标题、URL、文件名或屏幕文字。
- [ ] 测试覆盖 category 降级、未知 category、payload 脱敏。

## 验证方式

- Rust adapter / security / policy tests。
- Source scan。
- Mock active app presence smoke。
