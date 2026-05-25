# 0274 · 验证记录

## 状态

通过，真实快捷键触发仍需人工补验。

## 自动化验证

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：通过，181 个 Rust 测试全部通过。
- `PATH=/Users/bytedance/Git/ian/node_modules/.bin:$PATH vitest run`：通过。
- `PATH=/Users/bytedance/Git/ian/node_modules/.bin:$PATH tsc --noEmit`：通过。
- `PATH=/Users/bytedance/Git/ian/node_modules/.bin:$PATH vite build`：通过。

## 覆盖结果

- `find_ian_uses_peek_enter_and_settle_story_when_offscreen` 覆盖 offscreen 找回的 peek / enter / settle 多段 story，以及最终落点在 safe zone。
- `find_ian_reduced_motion_uses_single_gentle_return` 覆盖 reduced motion 单段低动效找回。
- `find_ian_entrance_moment_moves_only_before_cooldown` 覆盖连续触发不会堆叠移动。

## 桌面验证

- 真实 Tauri 桌面壳已启动，截图 `/tmp/ian-0273-0276-desktop.png` 确认 Ian 可见。
- 当前环境 `System Events` 辅助功能权限为 `false`，无法脚本化触发全局快捷键和观察完整 find story。

## 剩余风险

- 需要人工触发找回快捷键，观察 peek -> enter -> settle 的视觉节奏是否足够清楚，以及不会被 Dock / 菜单栏遮挡。
