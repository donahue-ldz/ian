# 0261 · Moment System Architecture Doc

## 问题 / 目标

`0252-0260` 已经把 Ian Moments 拆成一批 SDD，但总技术方案还没有同步 Moment System 的定位和边界。为了避免后续实现漂移，本 SDD 将 Moment System 写入 `docs/codex/` 技术约束，并在 `ian.md`、当前阶段和架构规则中建立索引。

## 当前产品阶段

v0.1.x / Life Feel Architecture。

## 产品范围

- 新增 `docs/codex/ian-moment-system.md`。
- 在 `docs/codex/ian-current-stage.md` 中标记 Moment System 是 v0.1.x 体验增强方向。
- 在 `docs/codex/ian-architecture-rules.md` 中明确 Moment Orchestrator 属于 Rust Core / BehaviorPolicy。
- 在 `ian.md` 中加入短索引，指向 Moment System 详细方案。
- 不修改产品代码。

## 明确不做

- 不实现 Moment Orchestrator。
- 不新增协议字段。
- 不修改资源包。
- 不改变 P0 范围。

## 用户体验

本文档不直接改变用户体验。它约束后续实现：Ian 的惊喜感应来自低频、可打断、受预算控制的小瞬间，而不是高频随机动画或聊天式打扰。

## 架构约束

- Moment System 属于 Rust Core 行为编排层。
- Moment 输入来自 `IanEvent`、`IanState`、冷却 / 预算和低敏本地上下文。
- Moment 输出必须仍是 `IanAction`。
- React 只执行动作，不决定核心行为、不维护长期 moment 状态。
- 所有 moment 必须尊重勿扰、reduced motion、用户交互状态和隐私边界。

## 数据 / 协议变化

本 SDD 不改协议。后续如实现需要新增 `IanMoment`、`moment.started` 诊断动作或 budget state，必须在对应实现 SDD 中明确记录。

## 隐私与安全边界

- Moment 不读取代码正文、diff、剪贴板、私聊、屏幕 OCR、全局键盘文本或完整终端输出。
- Moment 不上传数据。
- 记忆回响只能使用用户确认过的低敏 tags。
- Moment 预算和诊断只能记录低敏类型、计数和时间，不记录用户内容。

## 验收标准

- [ ] `docs/codex/ian-moment-system.md` 存在并说明 Moment System 的定位、架构、数据流、边界和 SDD 映射。
- [ ] `docs/codex/ian-current-stage.md` 提到 Moment System 属于 v0.1.x Life Feel 增强，而不是 P0 扩容。
- [ ] `docs/codex/ian-architecture-rules.md` 明确 Moment Orchestrator 属于 Rust Core / BehaviorPolicy，React 只执行 `IanAction`。
- [ ] `ian.md` 有短索引指向 `docs/codex/ian-moment-system.md`。
- [ ] 文档没有把社交、插件、开发者节奏或长期记忆提前变成 P0 用户可见能力。

## 验证方式

- 检查新增和修改文档是否存在。
- 使用 `rg` 检查 Moment System 索引、Rust Core 归属和隐私边界文字。
- 本 SDD 为文档更新，不需要启动 Tauri 桌面壳。
