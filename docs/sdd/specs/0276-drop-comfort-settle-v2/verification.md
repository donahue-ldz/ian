# 0276 · 验证记录

## 状态

通过，真实拖放手感需人工补验。

## 自动化验证

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：通过，181 个 Rust 测试全部通过。
- `PATH=/Users/bytedance/Git/ian/node_modules/.bin:$PATH vitest run`：通过，16 个测试文件 / 121 个测试全部通过。
- `PATH=/Users/bytedance/Git/ian/node_modules/.bin:$PATH tsc --noEmit`：通过。
- `PATH=/Users/bytedance/Git/ian/node_modules/.bin:$PATH vite build`：通过。

## 覆盖结果

- `drop_settle_returns_to_idle_and_does_not_stack_when_repeated` 覆盖 settle story、idle return 和连续 drop cooldown 不叠加反馈。
- `drag_end_and_run_around_include_settle_steps` 更新为检查 settle profile、`blush_puff` 和 idle return。
- `drag_start_and_end_have_distinct_touch_reactions` 继续覆盖 drop 后短气泡和位置目标。

## 桌面验证

- 真实 Tauri 桌面壳已启动，截图 `/tmp/ian-0273-0276-desktop.png` 确认 Ian 可见。
- 当前环境 `System Events` 辅助功能权限为 `false`，无法自动执行真实拖放手感和位置持久化连测。

## 剩余风险

- 需要人工拖放观察 settle 是否过长、是否影响继续拖动，以及重启后位置是否符合预期。
