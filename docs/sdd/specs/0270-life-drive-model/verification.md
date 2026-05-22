# 0270 · 验证记录

## 状态

通过。

## 自动化验证

- `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml --check`：通过。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml life_drive_updates_from_low_sensitive_events_without_exposing_scores`：通过。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：通过，175 个 Rust 测试全部通过。
- `PATH=/Users/bytedance/Git/ian/node_modules/.bin:$PATH tsc --noEmit`：通过。
- `PATH=/Users/bytedance/Git/ian/node_modules/.bin:$PATH vitest run`：通过，16 个测试文件 / 121 个测试全部通过。
- `PATH=/Users/bytedance/Git/ian/node_modules/.bin:$PATH vite build`：通过。

## 桌面验证

- 使用 `tauri dev --config '{"build":{"beforeDevCommand":""}}' --no-dev-server-wait` 启动真实 Tauri 桌面壳。
- `pgrep -fl 'target/debug/ian_desktop'` 确认桌面进程存在。
- `screencapture -x /tmp/ian-0270-0272-desktop-restarted.png` 确认桌面上 Ian 可见。

## 隐私与范围

- 未新增用户可见数值、分数或调试面板。
- 未持久化 Life Drive 内部状态。
- 未采集窗口内容、输入文本、文件路径或外部应用正文。
