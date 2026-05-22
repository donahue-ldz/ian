# 0103 · Simple Mood Calibration V1

## 问题 / 目标

把已有简单 Mood State 从占位推进到可感知但克制的校准版，只影响语气和少量动作权重。

## 当前产品阶段

v0.1.x Core Life。

## 产品范围

- 定义 calm、happy、sleepy、bored 等轻量状态的进入条件。
- 状态只由 Ian 自身低敏事件和时间驱动。
- 影响短句候选和动画权重。

## 明确不做

- 不展示心情数值。
- 不读取开发者上下文。
- 不做完整情绪系统。

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

- [ ] 用户点击和互动可以短期提高 happy 倾向。
- [ ] 长时间 idle 可以进入 bored 或 sleepy 倾向。
- [ ] Mood 变化不会直接弹通知，只影响后续行为。
- [ ] 测试覆盖状态转移、衰减和边界条件。

## 验证方式

- 运行与改动相关的 Rust / frontend 单元测试。
- 运行 npm run desktop:typecheck 或等价的 workspace typecheck。
- 涉及桌面可见行为时，必须启动真实 Tauri 桌面壳验收，并在 verification.md 记录窗口位置、动画、气泡、点击 / 拖动或设置操作结果。
- 如果桌面端无法验收，必须记录原因、替代验证和剩余风险。
