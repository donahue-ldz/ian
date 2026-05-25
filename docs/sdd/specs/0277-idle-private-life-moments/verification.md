# 0277 · 验证记录

## 状态

已实现并通过自动化验证。

## 实现结果

- Rust Core 新增 3 类 idle private life story：`idle_peek_around`、`idle_tiny_patrol`、`idle_pretend_innocent`。
- 每类 story 通过 `MomentOrchestrator` 使用独立冷却，并继续受全局 Moment 预算约束。
- 自动触发路径在低频 idle tick 窗口进入，候选顺序受 Life Drive 的 boredom / curiosity / energy 影响。
- DND、quiet/off、reduced motion、输入中会抑制或降级；诊断触发可直达。
- 前端设置面板已露出 3 个私生活 Moment 诊断按钮，便于桌面手动验收。

## 命令验证

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml private_life -- --nocapture`：4 passed。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：187 passed。
- `PATH=/opt/homebrew/bin:/Users/bytedance/Git/ian/node_modules/.bin:$PATH vitest run`（`apps/desktop`）：122 passed。
- `PATH=/opt/homebrew/bin:/Users/bytedance/Git/ian/node_modules/.bin:$PATH tsc --noEmit`（`apps/desktop`）：通过。
- `PATH=/opt/homebrew/bin:/Users/bytedance/Git/ian/node_modules/.bin:$PATH vite build`（`apps/desktop`）：通过。

## 桌面验证

- 已启动真实 Tauri desktop shell，并截图确认 Ian 可见：`/tmp/ian-0277-0278-desktop.png`。
- 当前 macOS `System Events` Accessibility 为 `false`，无法自动点击桌面设置面板里的诊断按钮。
- 替代验证：Rust Core 诊断触发单测覆盖 3 个新 Moment；前端设置面板和浏览器 fallback 测试覆盖新诊断入口与 action 回放。

## 剩余风险

- 未做 10-20 分钟真实 long-run 观察；自动触发频率目前由低频 tick 窗口、冷却和预算共同控制，需要后续实际使用中继续观察打扰感。
