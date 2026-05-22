# 0274 · Find Ian Peek And Return Story

## 问题 / 目标

找回 Ian 不能只是把窗口搬回来。它应该像 Ian 自己从屏幕边缘探头、出来、确认用户看到它。本 SDD 将 Find Ian 升级为 peek and return story。

## 当前产品阶段

v0.1.x Life Feel / Find Ian Story。

## 产品范围

- 找回时先定位到安全边缘或目标区域。
- 播放探头 / 入场 / 小确认 / 收尾四段 story。
- 找回后停在可拖动、可见、安全的位置。
- reduced motion 下改为低动效短提示。

## 明确不做

- 不实现全局键盘监听扩展。
- 不读取当前应用内容。
- 不跨屏乱跑。
- 不做通知式强提醒。

## 用户体验

用户找不到 Ian 时，触发找回后会看到 Ian 从边缘出现，像“我在这”，而不是生硬瞬移。

## 架构约束

- Find Ian 事件进入 Rust Core。
- Core 选择目标位置和 story。
- React 执行窗口移动、动画和气泡。

## 数据 / 协议变化

复用 safe zone、Micro Story 和 Motion Profile。必要时新增 find entrance intent。

## 隐私与安全边界

只使用屏幕几何和 Ian 自身状态，不读取屏幕内容。

## 验收标准

- [ ] 找回 Ian 后落点在当前可见屏幕 safe zone 内。
- [ ] 找回 story 至少包含 peek / enter / settle 三段。
- [ ] 连续找回不会造成动作堆积、闪烁或位置抖动。
- [ ] reduced motion 下使用低动效版本。
- [ ] 真实桌面上 Ian 不会被菜单栏、Dock 或屏幕边缘遮挡到不可交互。

## 验证方式

- Rust safe zone 和 story 测试。
- 前端移动和气泡测试。
- 真实 Tauri 桌面触发找回验收。
