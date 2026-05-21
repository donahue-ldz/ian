# 实现计划: Dialogue Provider Boundary

## 状态

已实现，已验证。

## 概要

把现有 Demo Dialogue 改为 trait 驱动的 provider boundary，不增加用户可见 BYOM 能力。

## 步骤

1. 添加 Rust RED 测试。
   - 覆盖 `DialogueEngine` 使用 provider boundary。
   - 覆盖输出裁剪。

2. 实现 provider trait 和 context。
   - 修改 `apps/desktop/src-tauri/src/domain/dialogue/providers/mod.rs`。
   - 修改 `apps/desktop/src-tauri/src/domain/dialogue/providers/demo.rs`。

3. 改造 `DialogueEngine`。
   - 修改 `apps/desktop/src-tauri/src/domain/dialogue/dialogue_engine.rs`。
   - 通过 trait object 或泛型调用 provider。

4. 验证并记录。
   - `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml dialogue`
   - `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
   - 更新 `verification.md`。

## 预计文件改动

- `apps/desktop/src-tauri/src/domain/dialogue/dialogue_engine.rs`
- `apps/desktop/src-tauri/src/domain/dialogue/dialogue_policy.rs`
- `apps/desktop/src-tauri/src/domain/dialogue/providers/mod.rs`
- `apps/desktop/src-tauri/src/domain/dialogue/providers/demo.rs`
- `docs/sdd/specs/0003-dialogue-provider-boundary/verification.md`

## 风险与回滚

风险是过早抽象 provider。限制为一个小 trait 和 context，避免配置系统。回滚时可恢复直接调用 Demo Provider。

