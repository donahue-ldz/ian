# 0273 · 验证记录

## 状态

通过，仍需人工补一次真实鼠标手感观察。

## 自动化验证

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：通过，181 个 Rust 测试全部通过。
- `PATH=/Users/bytedance/Git/ian/node_modules/.bin:$PATH vitest run`：通过，16 个测试文件 / 121 个测试全部通过。
- `PATH=/Users/bytedance/Git/ian/node_modules/.bin:$PATH tsc --noEmit`：通过。
- `PATH=/Users/bytedance/Git/ian/node_modules/.bin:$PATH vite build`：通过。

## 覆盖结果

- `pointer_chase_is_a_tease_story_with_discover_chase_and_settle` 覆盖 discover / chase / settle 顺序，且 movement 数量为 3。
- `pointer_chase_uses_life_drive_to_change_chase_strength` 覆盖 Life Drive 对 chase 强度的影响。
- 既有 `pointer_chase_blocks_when_distance_or_user_context_is_wrong`、`pointer_chase_does_not_interrupt_active_perimeter_patrol` 覆盖距离、交互和状态 gate。

## 桌面验证

- 启动真实 Tauri 桌面壳：`tauri dev --config '{"build":{"beforeDevCommand":""}}' --no-dev-server-wait`。
- `pgrep -fl 'target/debug/ian_desktop'` 确认桌面进程存在。
- `screencapture -x /tmp/ian-0273-0276-desktop.png` 确认 Ian 在真实桌面可见。
- `osascript -e 'tell application "System Events" to get UI elements enabled'` 返回 `false`，当前环境没有辅助功能权限，无法可靠自动化真实鼠标移出触发 chase。

## 剩余风险

- 真实鼠标快速移出后的主观手感仍需人工观察；自动化已覆盖 Core 决策、边界和动作序列。
