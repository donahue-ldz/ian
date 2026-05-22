# 0141 · Git Dirty State Gentle Reaction

## 问题 / 目标

让 Ian 对工作区 dirty / clean 摘要有温柔反应，但避免变成 Git 状态通知器。

## 当前产品阶段

v0.2 Developer Rhythm。

## 产品范围

- 只读取是否有未提交变更和数量级。
- 低频提醒保存工作。
- clean 时可轻微庆祝。

## 明确不做

- 不显示文件列表。
- 不读取文件名或内容。
- 不主动催提交。

## 用户体验

本 SDD 可以推进 Developer Rhythm，但必须坚持授权、摘要事件、低频反应和不读取代码正文。

## 架构约束

- Rust Core 继续作为行为大脑；React 只负责渲染、设置 UI、动画播放和执行 IanAction。
- 所有外部输入必须经过 IanEvent、Security Gate 或既有设置 command。
- 高敏能力必须默认关闭，并在权限中心明确说明。
- Adapter 只能产出摘要事件，不得直接控制动画或写长期状态。
- Resource Pack 和插件相关能力不得执行未授权脚本或访问网络。

## 数据 / 协议变化

默认不新增 IanEvent / IanAction / IanState 字段。若实现必须变更协议、schema 或持久化格式，必须先更新 spec、plan 和 decisions.md，并记录迁移和回滚策略。

## 隐私与安全边界

- 默认不读取代码正文、diff、剪贴板、私聊、屏幕 OCR、全局键盘文本或完整终端输出。
- 只允许本 SDD 明确列出的低敏摘要进入事件或持久化。
- API key、用户正文、路径、远端消息和敏感 payload 必须脱敏或默认不保存。
- 默认本地优先；任何联网能力必须由用户明确配置或授权。

## 验收标准

- [ ] dirty 摘要不含文件路径。
- [ ] 反应短且低频。
- [ ] clean 反应不打扰。
- [ ] 用户可关闭 Git 反应。

## 验证方式

- 运行本 SDD 相关的最小 Rust / frontend 测试。
- 运行 workspace 类型检查或记录无法运行原因。
- 涉及桌面可见行为时，必须在真实 Tauri 桌面壳中验收。
- 涉及安全 / 隐私时，必须包含拒绝路径、脱敏和默认关闭测试。
- verification.md 必须记录实际命令、结果、失败项、跳过项和剩余风险。
