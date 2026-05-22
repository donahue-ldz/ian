# 0200 · 已确认记忆用于短对话

## 问题 / 目标

让 Ian 在短气泡或 Demo Dialogue 中只使用用户确认过的小记忆，增强连续陪伴感。

## 当前产品阶段

v0.1.x Memory Prep。

## 产品范围

- 从 confirmed memory 读取低敏 tags。
- 把少量 tags 注入 dialogue context。
- 控制气泡长度和使用频率。

## 明确不做

- 不让 LLM 读取候选未确认内容。
- 不拼接原始聊天。
- 不让记忆主导对话。

## 用户体验

Ian 偶尔提到用户确认过的小偏好，例如“你喜欢我安静一点，我记得。”

## 架构约束

- Rust Core 继续作为 Ian 的行为大脑；React 只负责渲染、设置 UI、动画播放和执行 IanAction。
- 所有外部输入必须进入 IanEvent，并经过既有 Security / Permission / Sanitizer 边界。
- 不得把未来阶段能力提前做成默认开启的 P0 用户可见功能。
- 涉及桌面窗口、动画、气泡、拖动、快捷键或 Tauri API 的能力，必须真实 Tauri 桌面端验收。

## 数据 / 协议变化

实现前必须盘点现有 IanEvent / IanAction / IanState、config、SQLite schema 和资源包 manifest。若需要新增协议、配置或 migration，必须在 decisions.md 中记录兼容和回滚策略；若不需要变更，也必须在 verification.md 中确认。

## 隐私与安全边界

- 默认本地优先，不上传用户数据。
- 不读取代码正文、diff、剪贴板、私聊、屏幕 OCR、全局键盘文本或完整终端输出，除非本 SDD 明确授权且默认关闭。
- API key、路径、用户正文和敏感 payload 不得进入普通日志或导出。
- 新增能力必须可关闭，并明确记录默认状态。

## 验收标准

- [ ] 仅 confirmed 记忆可进入 dialogue context。
- [ ] 未确认 candidate 不被使用。
- [ ] 输出仍短、角色化、bubble-friendly。
- [ ] 记忆缺失时对话正常退化。

## 验证方式

- 运行本 SDD 相关的最小 Rust / frontend 测试。
- 运行类型检查；如无法运行，记录原因。
- 涉及桌面可见行为时，启动真实 Tauri 桌面壳验收，记录步骤和结果。
- 涉及安全 / 隐私时，覆盖拒绝路径、默认关闭和脱敏检查。
- verification.md 必须记录实际命令、结果、失败项、跳过项和剩余风险。
