# 0075 验证记录

## 2026-05-21

- 先写失败测试：`autonomous_roam_uses_visible_short_path_and_walk_animation` 在实现前失败，缺少 `walk` 动画和短路径移动。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml domain::behavior::behavior_engine`：通过，27 个行为引擎测试通过。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：通过，96 个 Rust 测试通过。
- `npm run desktop:test`：通过，11 个测试文件、53 个测试通过。
- `npm run desktop:typecheck`：通过。
- `npm run desktop:build`：通过。
- `git diff --check`：通过。

## 验收结论

通过。普通/活泼模式下自动游走会输出 `walk` 动画、两段短路径 `movement.move_to` 和 `idle` 回落；用户交互和安静保护仍生效。
