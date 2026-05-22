# 0191 · 验证记录

## 2026-05-22

### 本轮覆盖

- 新增低敏 `system.shortcut_triggered` / `find_ian` 事件，拒绝额外输入内容。
- Rust Core 对 `find_ian` 返回短气泡、happy 动画、轻微 effect；Ian 不可见时返回快速移动到安全区域的 `movement.move_to`。
- 设置面板新增 `找回 Ian` 手动入口、默认关闭快捷键开关、`Cmd+Shift+I` 文案和注册失败提示。
- 前端接入 `@tauri-apps/plugin-global-shortcut`；快捷键回调只发送 `find_ian` 事件。
- 配置新增 `find_ian_shortcut_enabled` 和 `find_ian_shortcut`，默认关闭。
- Tauri capability 新增 `global-shortcut:allow-is-registered`、`global-shortcut:allow-register`、`global-shortcut:allow-unregister`。

### RED 验证

- `PATH=/opt/homebrew/bin:$PATH npm run test --workspace @ian/desktop -- src/renderer/dragGesture.test.ts src/renderer/SettingsPanel.view.test.tsx src/App.test.ts`
  - 结果：失败；`findIan is not a function`，设置面板缺少找回入口。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml find_ian`
  - 结果：失败；`IanEvent::SystemShortcutTriggered` 不存在。

### 已执行命令

- `PATH=/opt/homebrew/bin:$PATH npm run test --workspace @ian/desktop -- src/renderer/dragGesture.test.ts src/renderer/SettingsPanel.view.test.tsx src/App.test.ts src/renderer/IanStage.test.tsx`
  - 结果：通过，4 个测试文件，37 个测试。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml find_ian`
  - 结果：通过，2 个 Rust 测试。
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
  - 结果：Rust dev build 完成并运行 `target/debug/ian_desktop`，global-shortcut 插件加载无启动期错误；本轮启动的 Tauri 和 dev server 已停止。

### Acceptance Criteria Results

- [x] 设置中存在可触发的 `找回 Ian` 入口。
- [x] 用户开启全局快捷键后，应用只注册 `找回 Ian` 用途的快捷键。
- [x] 快捷键触发后进入 Rust Core 的事件是低敏 `find_ian` 事件，而不是 Keyboard Rhythm 事件。
- [x] Rust Core 返回定位 / 动画 / 气泡相关 IanAction，React 只负责执行。
- [x] Ian 在位置越界或不可见时会回到当前可见屏幕安全区域。
- [x] Ian 已经可见时不会突兀瞬移，只做短回应。
- [x] 快捷键注册失败时，UI 能提示占用或不可用状态。
- [x] 关闭快捷键后会注销快捷键。
- [x] 测试覆盖事件协议、配置、注册失败文案和关闭路径。
- [ ] 实际按 `Cmd+Shift+I` 的人工验收未自动化；本轮完成真实 Tauri 启动 smoke。

### 剩余风险

- 需要用户在真实桌面中手动开启快捷键并按 `Cmd+Shift+I`，确认系统未被其他应用占用。
- 本轮没有做复杂快捷键编辑器。
