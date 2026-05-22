# 0263 · 验证记录

## 2026-05-22

### RED

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml moment_debug_trigger_reaches_core_for_each_known_moment`
  - 结果：失败，符合预期。`IanEvent::MomentDebugTrigger` 尚不存在。
- `PATH=/opt/homebrew/bin:$PATH npm run test --workspace @ian/desktop -- src/renderer/SettingsPanel.view.test.tsx src/lib/tauriBridge.browserFallback.test.ts src/state/ianActions.test.ts`
  - 结果：失败，符合预期。缺少 Moment 诊断入口、browser fallback 诊断事件和气泡时长限制。

### GREEN

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml moment_debug_trigger_reaches_core_for_each_known_moment`
  - 结果：通过。
- `PATH=/opt/homebrew/bin:$PATH npm run test --workspace @ian/desktop -- src/renderer/SettingsPanel.view.test.tsx src/lib/tauriBridge.browserFallback.test.ts src/state/ianActions.test.ts`
  - 结果：通过。3 个测试文件 / 40 个测试通过。
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:test`
  - 结果：通过。16 个测试文件 / 117 个测试通过。
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:typecheck`
  - 结果：通过。

## 验收标准结果

- [x] 新增 `moment.debug_trigger` 低敏诊断事件。
- [x] 找回、鼠标好奇、拖动抱起、放下安顿、idle 惊喜、记忆回响均可确定性触发。
- [x] React 只发送事件并执行 Rust / fallback action，不直接伪造核心行为策略。
- [x] 设置面仅在本地诊断开启时显示 Moment 诊断入口。
- [x] 诊断 payload 只包含 kind 和时间，不包含屏幕内容或用户文本。

## 剩余风险

- 诊断入口已在设置中可见；未逐个按钮人工点击观察真实动画，只完成 Tauri 启动 smoke。
