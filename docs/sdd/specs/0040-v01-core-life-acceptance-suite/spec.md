# Spec: v0.1 Core Life Acceptance Suite

## 状态

已实现，待验收。

## 问题 / 目标

0021-0039 会形成 v0.1 核心生命感。0040 目标是建立一套可重复验收流程，验证 Ian 是否真的具备“住在桌面上”的最小生命体验，而不是只靠零散单测。

## 当前产品阶段

v0.1.x Acceptance。

## 产品范围

- 汇总核心生命感验收场景：启动、移动、双击跑动、待机游走、休息、好奇、亲近、回 anchor、安静时段、设置。
- 建立自动化 smoke 或半自动验收脚本。
- 明确人工视觉验收清单。
- 更新 SDD 验收标准和版本通过条件。

## 明确不做什么

- 不把 Developer Rhythm 作为 v0.1 通过条件。
- 不要求完整长期记忆、Feishu、Pet Visit 或插件系统。
- 不追求最终美术。
- 不做性能基准平台。

## 用户体验

通过 0040 后，Ian 应该可以被判断为“v0.1 核心生命感可用”：会动、会停、会回应、可控、不会明显打扰。

## 架构约束

- 验收套件覆盖 Rust Core、React 执行层、Tauri 桌面 smoke。
- 验收结果记录在对应 verification 文档。
- 自动化不能依赖外部服务。

## 数据 / 协议变化

不新增产品协议。可新增测试工具或 smoke 脚本。

## 隐私与安全边界

验收只使用本地低敏输入，不读取用户真实应用、键盘、终端或代码内容。

## 验收标准

- [ ] 有一份 v0.1 core life acceptance checklist。
- [ ] 自动化覆盖核心 Rust behavior / scheduler / storage / security 回归。
- [ ] 前端覆盖 action execution、resource、settings、pointer interaction。
- [ ] Tauri 或 Browser smoke 覆盖启动、点击、双击、移动、休息、设置。
- [ ] 验收明确排除 Developer Rhythm 和未来阶段能力。

## 验证方式

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm run desktop:test`
- `npm run desktop:typecheck`
- `npm run desktop:build`
- Browser / Tauri smoke checklist。
