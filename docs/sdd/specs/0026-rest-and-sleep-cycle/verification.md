# 验证记录: Rest And Sleep Cycle

## 状态

已实现，待用户验收。

## 自动验证

- [x] Scheduler tests - sleep cadence and run non-interruption tests passed.
- [x] Behavior tests - `click_wakes_a_sleeping_ian_before_showing_affection` passed.
- [x] Behavior tests - `time_tick_does_not_roam_or_sleep_while_user_is_interacting` and `sleep_candidate_wins_over_lower_priority_roam` passed.
- [x] Runtime tests - `interaction_lifecycle_events_suppress_autonomous_tick_actions` passed.
- [x] `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` - 66 tests passed.
- [x] `npm run desktop:test` - 6 files / 21 tests passed.

## 备注

Sleep / wake remains driven by Ian local tick and direct interaction only. 验收修复后，拖拽、气泡输入和 run-around 期间不会触发 sleep/rest。
