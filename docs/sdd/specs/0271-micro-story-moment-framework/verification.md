# 0271 · 验证记录

## 状态

通过。

## 自动化验证

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml micro_story_outputs_ordered_beats_and_reduced_motion_variant`：通过。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml pointer_debug_moment_uses_story_and_playful_motion_profile`：通过。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml mouse_near`：通过，4 个鼠标靠近回归测试全部通过。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：通过，175 个 Rust 测试全部通过。
- `PATH=/Users/bytedance/Git/ian/node_modules/.bin:$PATH vitest run`：通过，16 个测试文件 / 121 个测试全部通过。

## 桌面验证

- 真实 Tauri 桌面壳已启动，桌面截图 `/tmp/ian-0270-0272-desktop-restarted.png` 确认 Ian 可见。
- 由于当前 macOS 未授予 `osascript` 辅助功能权限，无法在本次自动化中可靠模拟真实鼠标靠近和点击；已用 Rust Core 行为测试覆盖 story 顺序、cooldown、交互中不打断和 reduced motion 降级。

## 剩余风险

- Story beat 目前通过 `IanAction` 顺序表达，`pause` 仍是内部节拍元数据，没有独立协议动作；后续如果需要更强的节奏控制，应在协议层补 `Delay/Wait` 类动作或在前端执行器支持 beat metadata。
