# 0101 · Local Interaction Journal V1

## 问题 / 目标

建立轻量本地互动日志，为后续 mood、bond、reminder 和长期记忆提供可追溯基础，但 P0 不展示日志。

## 当前产品阶段

v0.1 Architecture Baseline。

## 产品范围

- 记录低敏 Ian 自身互动事件。
- 写入 SQLite 或现有 storage skeleton。
- 提供 repository 测试。

## 明确不做

- 不记录代码内容、剪贴板、私聊或屏幕文本。
- 不做用户可见时间线。
- 不生成长期记忆总结。

## 用户体验

用户感知到的是 Ian 的基础生命感和可控性继续变稳：行为更清楚、更可恢复、更少打扰；任何新增能力都保持短句、桌面宠物、低敏和本地优先。

## 架构约束

- Rust Core 继续作为行为大脑；React 只负责渲染、动画播放、设置 UI 和执行 IanAction。
- 外部或桌面输入必须先转换为 IanEvent 或通过既有设置 command 进入 Rust / Storage 边界。
- Resource Pack 只描述本地资源，不执行脚本、不访问网络。
- 不把 v0.2 / v0.3 能力提前做成 P0 用户可见功能。

## 数据 / 协议变化

除本 SDD 实现时明确记录到 decisions.md 的最小必要变更外，默认不新增 IanEvent / IanAction / IanState 字段。若实现发现必须改协议，先更新本 spec 和 plan，再实现。

## 隐私与安全边界

- 默认不读取代码内容、剪贴板、屏幕文字、私聊内容或全局键盘。
- 只使用 Ian 自身窗口事件、本地配置、bundled resource pack 和用户明确设置。
- 所有持久化内容必须是低敏本地数据，并避免记录用户正文或敏感值。

## 验收标准

- [ ] 点击、拖动、资源包切换、提醒确认等低敏事件可以写入本地 interaction journal。
- [ ] 事件 payload 有大小限制和敏感源分类。
- [ ] repository 支持追加和按时间读取最近事件。
- [ ] 默认不上传、不联网、不展示。

## 验证方式

- 运行与改动相关的 Rust / frontend 单元测试。
- 运行 npm run desktop:typecheck 或等价的 workspace typecheck。
- 涉及桌面可见行为时，必须启动真实 Tauri 桌面壳验收，并在 verification.md 记录窗口位置、动画、气泡、点击 / 拖动或设置操作结果。
- 如果桌面端无法验收，必须记录原因、替代验证和剩余风险。
