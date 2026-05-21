# 0080 验证记录

## 2026-05-21

- 先写失败测试：
  - `mouse_chase_candidate_accepts_only_low_sensitive_coordinates` 覆盖事件反序列化和拒绝敏感额外字段。
  - `pointer_chase_coordinates_are_allowed_as_local_mouse_input` 覆盖权限门。
  - `pointer_chase_moves_toward_nearby_cursor_without_jumping_to_it` 在实现前失败，缺少追逐动作。
  - `pointer_chase_blocks_when_distance_or_user_context_is_wrong` 覆盖距离、输入和冷却阻断。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml pointer_chase`：通过，3 个测试通过。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml mouse_chase_candidate`：通过，1 个测试通过。
- `npm run desktop:typecheck`：通过。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：通过，100 个 Rust 测试通过。
- `npm run desktop:test`：通过，11 个测试文件、56 个测试通过。
- `npm run desktop:build`：通过。
- `git diff --check`：通过。

## 验收结论

通过。桌面模式会每 10 秒发送一次低敏鼠标追逐候选事件；Rust Core 在合适条件下让 Ian 朝鼠标方向跑一小段，并通过玩闹冷却避免连续追逐。
