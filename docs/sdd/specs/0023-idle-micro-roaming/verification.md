# 验证记录: Idle Micro Roaming

## 状态

已实现，待用户验收。

## 自动验证

- [x] Scheduler tests - existing mode cadence and no-run-interrupt tests passed.
- [x] Behavior tests - `time_tick_idle_roam_moves_without_interrupting_run` passed.
- [x] `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` - 49 tests passed.
- [x] `npm run desktop:test` - 6 files / 18 tests passed.

## 备注

Idle roam is intentionally low-frequency and disabled during run / quiet mode.
