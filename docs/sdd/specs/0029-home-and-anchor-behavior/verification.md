# 验证记录: Home And Anchor Behavior

## 状态

已实现，待用户验收。

## 自动验证

- [x] Storage tests - position and `home_anchor` round-trip; older config defaults anchor to saved position.
- [x] Behavior tests - run-around final movement returns to `home_anchor`.
- [x] `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` - 49 tests passed.
- [x] `npm run desktop:test` - 6 files / 18 tests passed.
- [x] Browser Playwright smoke - double-click returned to idle at anchor transform.
