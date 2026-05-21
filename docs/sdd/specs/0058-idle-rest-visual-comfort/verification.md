# Verification: Idle Rest Visual Comfort

## 状态

已验证。

## 需要记录的验证

- [x] Rust behavior tests
- [x] `npm run desktop:test`
- [x] `npm run desktop:typecheck`
- [x] 5 分钟 idle smoke
- [x] reduced motion / quiet mode 截图或录屏

## 结果

### TDD RED

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml time_tick_returns_predictable_idle_rest_sleep_visual_states`
  - 失败原因：45 秒 tick 仍返回 `walk`，没有 `rest`。
- `npm run desktop:test -- IanStage.test.tsx`
  - 失败原因：缺少 `data-behavior-mode="quiet"`，CSS 缺少 `rest`、sleep 姿态、quiet 和 reduced-motion 规则。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml cadence_windows_survive_fifteen_second_desktop_ticks`
  - 失败原因：15 秒桌面 tick 可能错过整除秒，52 秒未进入 `rest`。

### 自动验证

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 通过：80 passed, 0 failed。
- `npm run desktop:test`
  - 通过：9 files / 37 tests。
- `npm run desktop:typecheck`
  - 通过。
- `npm run desktop:build`
  - 通过，Vite build 完成。

### Browser Smoke

- 本地地址：`http://127.0.0.1:1420/?v=0058-smoke`
- 5 分钟 idle smoke：`screenshots/five-minute-smoke.json`
  - 31 个采样。
  - 采样动画包含 `idle`、`rest`、`sleep`。
  - `invalidSamples: []`，没有空白 sprite、异常气泡或 0 尺寸 surface。
- 截图：
  - `screenshots/idle-default.jpg`
  - `screenshots/rest-default.jpg`
  - `screenshots/sleep-default.jpg`
  - `screenshots/quiet-mode.jpg`

### 结论

0058 验收标准已覆盖。`rest` 是 P0 轻休息视觉状态，不引入完整 Mood / 作息系统。
