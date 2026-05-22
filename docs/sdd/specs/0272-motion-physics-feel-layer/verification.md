# 0272 · 验证记录

## 状态

通过。

## 自动化验证

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml pointer_debug_moment_uses_story_and_playful_motion_profile`：通过。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：通过，175 个 Rust 测试全部通过，并导出 `MotionProfile` binding。
- `PATH=/Users/bytedance/Git/ian/node_modules/.bin:$PATH vitest run src/state/ianActions.test.ts src/renderer/IanStage.test.tsx src/lib/tauriBridge.browserFallback.test.ts`：通过，3 个测试文件 / 40 个测试全部通过。
- `PATH=/Users/bytedance/Git/ian/node_modules/.bin:$PATH tsc --noEmit`：通过。
- `PATH=/Users/bytedance/Git/ian/node_modules/.bin:$PATH vite build`：通过。

## 桌面验证

- 真实 Tauri 桌面壳已启动，进程 `target/debug/ian_desktop` 存在。
- 桌面截图 `/tmp/ian-0270-0272-desktop-restarted.png` 确认 Ian 在桌面上渲染可见。

## 剩余风险

- 本次自动化可验证 motion profile 的协议、状态归约、CSS 绑定和桌面启动，但无法在无辅助功能权限下自动拖动鼠标触发所有桌面手感分支。
- 真实手感仍建议人工观察 `playful` 弧线和 `settle` 落地缓冲是否足够明显。
