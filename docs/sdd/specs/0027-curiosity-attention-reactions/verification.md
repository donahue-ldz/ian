# 验证记录: Curiosity Attention Reactions

## 状态

已实现，待用户验收。

## 自动验证

- [x] Behavior tests - `mouse_near_returns_a_light_happy_reaction`, `mouse_near_attention_respects_cooldown_and_recovers_after_window`, and `mouse_leave_returns_attention_to_idle_without_global_tracking` passed.
- [x] `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` - 66 tests passed.
- [x] `npm run desktop:typecheck` - passed.
- [x] Source scan for global mouse APIs - no production matches in app source; only the behavior test name contains `global_tracking`.
- [x] Browser Playwright smoke - pointer-visible Ian button remains interactive after run and click reactions.
