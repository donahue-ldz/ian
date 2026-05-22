# 0107 · Demo Dialogue Character Pack V2

## 问题 / 目标

扩充 Demo Dialogue 的本地短句包，让无 API key 时 Ian 也更像有性格的小生命，而不是固定模板回复。

## 当前产品阶段

v0.1.x Product Feel。

## 产品范围

- 补充中文优先短句候选。
- 按 mood / bond / 场景选择本地短句。
- 保持回复短、可气泡显示。

## 明确不做

- 不接 LLM。
- 不保存聊天正文。
- 不做长对话上下文。

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

- [ ] 无 API key 时，常见点击 / 问候 / 陪伴类输入有多样短句。
- [ ] 短句不自称 AI assistant、Copilot 或工具。
- [ ] 回复长度适合当前气泡，不溢出。
- [ ] 测试覆盖中文优先、长度限制和禁用口吻。

## 验证方式

- 运行与改动相关的 Rust / frontend 单元测试。
- 运行 npm run desktop:typecheck 或等价的 workspace typecheck。
- 涉及桌面可见行为时，必须启动真实 Tauri 桌面壳验收，并在 verification.md 记录窗口位置、动画、气泡、点击 / 拖动或设置操作结果。
- 如果桌面端无法验收，必须记录原因、替代验证和剩余风险。
