# 0240 · 桌面 smoke 清单 v2

## 问题 / 目标

补充 post-0231 桌面壳验收清单，覆盖动画、气泡、减少动画、勿扰和未来能力默认关闭。

## 当前产品阶段

P0 / MVP + v0.1 Architecture Baseline。

## 产品范围

- 只做 post-0231 的小型 hardening / readiness 切片。
- 保持 Ian 是本地优先桌面数字生命，而不是 AI assistant 或开发工具。
- 以文档、审计、测试或默认关闭 gate 为主，不新增 P0 用户可见未来能力。

## 明确不做什么

- 不启用 Feishu、Pet Visit、插件运行时、全局键盘监听、代码读取、终端全文读取或远端资源导入。
- 不把 Mood / Bond / Memory / Reminder 做成完整产品。
- 不新增复杂设置页、仪表盘、开发者工作流或营销页面。

## 用户体验

- 用户可见体验继续围绕 Ian 的轻量生命感、舒适反馈和可控低打扰。
- 未来能力只作为默认关闭的骨架或 readiness 文档出现。

## 架构约束

- 外部输入仍必须先成为 IanEvent。
- 行为输出仍必须通过 IanAction。
- Rust Core 保持行为、策略、隐私和状态边界；React 只执行动作和渲染。
- 资源包、adapter、storage、security 边界不得被绕过。

## 数据 / 协议变化

- 默认不新增持久化 schema。
- 如实现发现需要协议或配置变更，必须先记录到 decisions.md，并保证向后兼容。

## 隐私与安全边界

- 不读取代码正文、diff、终端输出、剪贴板、屏幕 OCR、私聊正文或任意文件内容。
- 新文档和测试必须明确 default-off、local-first、low-sensitive payload。

## 验收标准

- [ ] 已创建并维护本 SDD packet 的 spec、plan、decisions、verification。
- [ ] 相关实现或文档只覆盖本 SDD 的目标，不引入 P0 禁止能力。
- [ ] 自动测试覆盖本 SDD 的客观边界。
- [ ] 如未涉及新桌面可见行为，verification 明确记录桌面壳启动作为回归验收。
- [ ] verification.md 记录实际命令、结果、失败项、跳过项和剩余风险。

## 验证方式

- 前端目标测试或 acceptance docs 测试。
- Rust 目标测试或全量 cargo test。
- typecheck、build、format、diff check。
- 启动真实 Tauri 桌面壳做回归验收。
