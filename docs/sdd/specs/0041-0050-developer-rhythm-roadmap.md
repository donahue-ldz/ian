# 0041-0050 SDD 拆分总览: v0.2 Developer Rhythm 产品化

## 状态

已实现，待验收。

## 拆分原则

0040 之前已经把 v0.1 核心生命感推进到验收套件。根据 `ian.md` 的阶段路线，下一阶段进入 v0.2 Developer Rhythm。

0016-0020 已经建立 Git、build/test、键盘节奏、活动应用类别和 Developer Rhythm Policy 的早期边界。本批不重复做 adapter skeleton，而是把 v0.2 做成可控、可验收、低打扰的产品能力：授权引导、工作区绑定、低敏事件反应、节奏策略、隐私审计和整体验收。

## 顺序

| 编号 | Packet | 阶段 | 目标 |
| --- | --- | --- | --- |
| 0041 | `0041-developer-rhythm-onboarding` | v0.2 Developer Rhythm | 用清晰授权引导开启开发者节奏能力，默认关闭。 |
| 0042 | `0042-project-workspace-binding` | v0.2 Developer Rhythm | 绑定当前项目工作区，只读取低敏 workspace 元数据。 |
| 0043 | `0043-git-rhythm-reactions` | v0.2 Developer Rhythm | 将 Git 低敏事件转成克制、低频的 Ian 反应。 |
| 0044 | `0044-build-test-summary-ingestion` | v0.2 Developer Rhythm | 接收 build/test 摘要输入，不读取终端全文。 |
| 0045 | `0045-developer-reaction-pack` | v0.2 Developer Rhythm | 建立开发者节奏专用短句、动作和冷却策略。 |
| 0046 | `0046-keyboard-rhythm-consent-hardening` | v0.2 Developer Rhythm | 加固键盘节奏 opt-in、采集停止和 payload 脱敏。 |
| 0047 | `0047-active-app-disturbance-policy` | v0.2 Developer Rhythm | 用粗粒度 app category 降低打扰，不做 window-aware。 |
| 0048 | `0048-developer-snooze-and-busy-policy` | v0.2 Developer Rhythm | 用户可暂停开发者反应，忙碌时主动降低存在感。 |
| 0049 | `0049-developer-rhythm-privacy-audit` | v0.2 Developer Rhythm | 建立隐私审计和本地诊断，证明未读取敏感内容。 |
| 0050 | `0050-v02-developer-rhythm-acceptance-suite` | v0.2 Acceptance | 建立 v0.2 Developer Rhythm 端到端验收套件。 |

## 依赖关系

```txt
0041 -> 0042
0041 -> 0046
0042 -> 0043
0042 -> 0044
0043 + 0044 -> 0045
0047 依赖 0020 / 0033 / 0038
0048 依赖 0041 / 0045 / 0047
0049 依赖 0041-0048
0050 依赖 0041-0049
```

## 防漂移规则

- v0.2 仍然不是 Copilot，不做代码分析助手。
- Git adapter 不能读取 diff、代码正文、commit message 全文或远程凭据。
- Build/test 能力只接收摘要，不读取 stdout/stderr 全文，不自动运行命令。
- Keyboard rhythm 必须显式 opt-in，payload 不含 key、text、shortcut。
- Active app presence 只允许粗粒度 category，不读取窗口标题、URL、文档名或屏幕文字。
- Developer Rhythm 反应必须低频、有冷却、可暂停，不能变成通知中心。
