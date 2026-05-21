# 验证记录: Idle Micro Roaming

## 状态

已实现，待用户验收。

## 自动验证

- [x] Scheduler tests - existing mode cadence and no-run-interrupt tests passed.
- [x] Behavior tests - `time_tick_idle_roam_moves_without_interrupting_run` passed.
- [x] Behavior tests - `time_tick_does_not_roam_or_sleep_while_user_is_interacting` passed.
- [x] Scheduler tests - `scheduler_does_not_interrupt_active_user_interaction` passed.
- [x] Runtime tests - `interaction_lifecycle_events_suppress_autonomous_tick_actions` passed.
- [x] `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` - 66 tests passed.
- [x] `npm run desktop:test` - 6 files / 21 tests passed.

## 备注

Idle roam is intentionally low-frequency and disabled during run / quiet mode. 验收修复后，拖拽和气泡输入 active 也会阻止自主 roam。
