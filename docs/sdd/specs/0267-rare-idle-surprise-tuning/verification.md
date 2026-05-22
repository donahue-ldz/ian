# 0267 · 验证记录

## 2026-05-22

### 已执行检查

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 结果：通过。171 个 Rust 测试通过，覆盖 rare idle surprise、budget、cooldown、DND 和 reduced motion。
- `PATH=/opt/homebrew/bin:$PATH npm run test --workspace @ian/desktop -- src/lib/tauriBridge.browserFallback.test.ts`
  - 结果：通过。
- 真实 Tauri 桌面壳启动 smoke
  - 结果：通过。

## 验收标准结果

- [x] 诊断入口可稳定触发 `rare_idle_surprise`。
- [x] 正式路径保留低频窗口、per-kind cooldown 和全局 budget。
- [x] DND 与 reduced motion 会阻断或降级 idle surprise。
- [x] 单次 surprise 由短 animation/effect 组成，并回到 idle。
- [x] 不读取外部工作内容决定惊喜。

## 剩余风险

- 未做 10-20 分钟真实 long-run 观察；正式低频体验仍需人工补验。
