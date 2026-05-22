# Contributor Onboarding

开始前必须读：

1. `AGENTS.md`
2. `docs/codex/engineering-defaults.md`
3. `docs/codex/ian-architecture-rules.md`
4. `docs/codex/ian-current-stage.md`
5. `docs/codex/sdd-workflow.md`

工作方式：

- 非平凡改动必须维护 `docs/sdd/specs/<id>-<slug>/` 下的 `spec.md`、`plan.md`、`decisions.md`、`verification.md`。
- 实现必须限制在已批准 SDD 范围内。
- Ian 是本地优先桌面数字生命，不是 AI assistant、Copilot 或聊天皮肤。
- Rust Core 是行为大脑；React 只渲染和执行 `IanAction`。
- 所有外部输入必须经过 `IanEvent`、adapter、Security Gate 或明确设置 command。

桌面验收：

- 涉及窗口、动画、气泡、位置、鼠标交互、Tauri API 或桌面可见行为时，必须启动真实 Tauri 桌面壳。
- 浏览器预览只能补充，不能替代桌面验收。

隐私禁区：

- 默认不读取代码正文、diff、剪贴板、私聊、屏幕 OCR、窗口标题、终端全文或按键内容。
- 高敏能力默认关闭。
- API key 和敏感正文不得写入 config、日志、诊断或测试 fixture。
