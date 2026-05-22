# 0192 · SDD 台账校准

## 问题 / 目标

校准历史 SDD 状态，区分已完成、部分完成、仅 skeleton、待验收和过期重复文档，避免后续 AI 按错误台账继续开发。

## 当前产品阶段

SDD Governance。

## 产品范围

- 扫描现有 SDD 的 spec、decisions、verification。
- 标记重复编号、过期草稿、后续已覆盖但 verification 未回填的 SDD。
- 输出一份机器和人都能读懂的 SDD ledger。
- 不修改产品代码。

## 明确不做

- 不重写历史实现。
- 不删除 SDD 目录。
- 不伪造验收结果。

## 用户体验

开发者在继续 SDD 时能先看到准确台账：哪些可继续验收，哪些只是 skeleton，哪些必须补验。

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

- [ ] 台账列出每个 SDD 的编号、名称、状态、剩余风险和下一步。
- [ ] 重复或冲突编号被明确标注。
- [ ] 只读分析和文档更新不改变产品行为。
- [ ] 台账能解释 0160-0189 是最小骨架而非完整产品化能力。

## 验证方式

- 运行本 SDD 相关的最小 Rust / frontend 测试。
- 运行类型检查；如无法运行，记录原因。
- 涉及桌面可见行为时，启动真实 Tauri 桌面壳验收，记录步骤和结果。
- 涉及安全 / 隐私时，覆盖拒绝路径、默认关闭和脱敏检查。
- verification.md 必须记录实际命令、结果、失败项、跳过项和剩余风险。
