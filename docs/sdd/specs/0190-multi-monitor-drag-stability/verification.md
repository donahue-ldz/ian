# Verification: 多显示器拖动稳定性

## 2026-05-22

### 本轮覆盖

- 桌面端拖动路径改为只触发 Tauri 原生 `startDragging()`，pointer move 不再调用前端 `setPosition()`。
- 拖动结束继续读取 `getCurrentWindow().outerPosition()`，并通过 `save_window_position` 持久化真实窗口位置。
- 浏览器预览仍使用本地 `dragOffset`。
- 未新增 0190 专属 Tauri window 权限。

### RED 验证

- `PATH=/opt/homebrew/bin:$PATH npm run test --workspace @ian/desktop -- src/renderer/dragGesture.test.ts src/renderer/SettingsPanel.view.test.tsx src/App.test.ts`
  - 结果：失败；`findIan is not a function` 和设置面板缺少找回入口。该命令同时确认 0190 测试已不再依赖物理坐标换算。

### 已执行命令

- `PATH=/opt/homebrew/bin:$PATH npm run test --workspace @ian/desktop -- src/renderer/dragGesture.test.ts src/renderer/SettingsPanel.view.test.tsx src/App.test.ts src/renderer/IanStage.test.tsx`
  - 结果：通过，4 个测试文件，37 个测试。
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:typecheck`
  - 结果：通过。
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:test`
  - 结果：通过，12 个测试文件，89 个测试。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 结果：通过，148 个 Rust 测试。
- `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 结果：通过。
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:build`
  - 结果：通过，Vite production build 完成。

### 桌面验收

- 启动 dev server：`PATH=/opt/homebrew/bin:$PATH npm run dev --workspace @ian/desktop -- --host 127.0.0.1 --port 5175`
  - 结果：Vite 在 `http://127.0.0.1:5175/` ready。
- 启动真实 Tauri 桌面壳：`PATH=/opt/homebrew/bin:$PATH npm run tauri --workspace @ian/desktop -- dev --no-dev-server-wait --config '{"build":{"beforeDevCommand":"","devUrl":"http://127.0.0.1:5175"}}'`
  - 结果：Rust dev build 完成并运行 `target/debug/ian_desktop`，启动期无错误；本轮启动的 Tauri 和 dev server 已停止。

### Acceptance Criteria Results

- [x] 桌面端拖动开始后，只调用 Tauri 原生 `startDragging()` 作为窗口移动机制，不再在 pointer move 中同时调用 `setPosition()`。
- [x] 桌面端拖动结束后，使用 `getCurrentWindow().outerPosition()` 的真实窗口位置发送 `mouse.drag_end` 并调用 `save_window_position`。
- [x] 浏览器预览拖动仍使用本地 `dragOffset`，相关测试通过。
- [x] 现有 Tauri window capability 不新增 0190 所需权限。
- [ ] 三屏手动拖动验收未由自动化覆盖；本轮完成真实 Tauri 启动 smoke，仍需用户在当前三屏环境手动拖动确认跨屏体感。

### 剩余风险

- Tauri smoke 证明启动期可用，但没有自动模拟跨显示器物理拖动。
