# 0270 · 实施计划

## 步骤

1. 阅读 `moment_orchestrator.rs`、`behavior_engine.rs` 和现有 mood / bond skeleton，确认放置边界。
2. 新增 Rust Core 内部 Life Drive 类型，包含默认值、边界夹取、事件更新方法。
3. 将点击、拖动、放下、找回、idle tick 等低敏事件映射为驱动力变化。
4. 将 Life Drive 作为 Moment Orchestrator 的只读上下文。
5. 增加 Rust 单元测试，覆盖更新规则、DND / quiet / reduced motion 抑制。
6. 更新本 SDD 的 `verification.md`。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/life_drive.rs`
- `apps/desktop/src-tauri/src/domain/behavior/mod.rs`
- `apps/desktop/src-tauri/src/domain/behavior/behavior_engine.rs`
- `apps/desktop/src-tauri/src/domain/behavior/moment_orchestrator.rs`
- `docs/sdd/specs/0270-life-drive-model/verification.md`

## 受影响接口或模块

- Rust Core Behavior Engine
- Moment Orchestrator
- 内部行为策略上下文

## 兼容性说明

默认不迁移持久化数据。驱动力先以内存态存在，后续如需持久化必须独立 SDD。

## 验证命令或手动检查

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm run desktop:typecheck` 或等价 `tsc --noEmit`
- 真实 Tauri 桌面启动 smoke。

## 风险和回滚

- 风险：驱动力过早复杂化。
- 缓解：只做 5 个轻量值和明确事件更新，不引入复杂状态机。
- 回滚：让 Moment Orchestrator 忽略 Life Drive 上下文。
