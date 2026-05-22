# 0179 · Social Presence Readiness Gate

## 问题 / 目标

建立进入 v0.3 Social Presence 前的门禁，确保本地生命感和权限边界已稳定。

## 当前产品阶段

v0.3 Readiness Gate。

## 产品范围

- 检查权限中心、安全、白名单、visit skeleton。
- 输出风险。

## 明确不做

- 不实现真实社交。
- 不绕过 v0.2 门禁。
- 不默认联网。

## 用户体验

本 SDD 属于未来阶段准备或门禁，不得默认开启用户可见能力；只能建立文档、schema、skeleton、测试或默认关闭的边界。

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

- [ ] 所有社交 skeleton 默认关闭。
- [ ] 白名单和拒绝路径测试通过。
- [ ] 输出 v0.3 启动条件。

## 验证方式

- 运行本 SDD 相关的最小 Rust / frontend 测试。
- 运行 workspace 类型检查或记录无法运行原因。
- 涉及桌面可见行为时，必须在真实 Tauri 桌面壳中验收。
- 涉及安全 / 隐私时，必须包含拒绝路径、脱敏和默认关闭测试。
- verification.md 必须记录实际命令、结果、失败项、跳过项和剩余风险。
