# 实现计划: Local State 与基础设置持久化

## Spec

`docs/sdd/specs/0005-local-state-persistence/spec.md`

## 状态

草稿，等待用户确认。

## 概要

补齐 P0 local-first 最小状态能力，确保 position/config 可以初始化、保存、恢复，并保留 SQLite skeleton。

## 步骤

1. 梳理当前 storage / config 实现。
2. 明确 position 语义：窗口位置或窗口内 Ian 位置。
3. 修正保存和恢复链路。
4. 确保 `~/.ian/config.toml` 默认值稳定。
5. 确保 `~/.ian/ian.db` 和 migration skeleton 可初始化。
6. 增加轻量测试或脚本化 smoke check。
7. 更新 verification。

## 预计文件改动

- `apps/desktop/src-tauri/src/storage/*`
- `apps/desktop/src-tauri/src/desktop/commands.rs`
- `apps/desktop/src-tauri/src/protocol/state.rs`
- `apps/desktop/src/lib/position.ts`
- `apps/desktop/src/state/useIanActions.ts`
- `docs/sdd/specs/0005-local-state-persistence/verification.md`
- `docs/sdd/specs/0005-local-state-persistence/decisions.md`

## 验证命令

```bash
npm run desktop:typecheck
npm run desktop:build
cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml
```

手动或脚本：

```bash
ls ~/.ian/config.toml
ls ~/.ian/ian.db
```

## 风险

- 桌面窗口位置和 Ian 内部拖拽位置如果混用，会导致恢复语义不清。
- 自动化拖拽验证成本可能较高，可能需要人工 smoke test。

## 回滚说明

可回退 storage、position 和 command 相关改动，恢复 0001 的最小持久化 skeleton。

