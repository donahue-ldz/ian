# 0257 · 验证记录

## 2026-05-22

### RED

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml moment`
  - 结果：失败，符合预期。idle tick 没有 rare surprise moment、budget gate 和 reduced motion 阻断。

### GREEN

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml moment`
  - 结果：通过。覆盖 `rare_idle_surprise_uses_budget_and_respects_reduced_motion`。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 结果：通过。169 个 Rust 测试通过。
- `npm run desktop:test`
  - 结果：通过。16 个测试文件 / 114 个前端测试通过。
- `npm run desktop:typecheck`
  - 结果：通过。
- `npm run desktop:build`
  - 结果：通过。

## 验收标准结果

- [x] 长 idle 窗口可触发低频 `rare_idle_surprise`。
- [x] 全局 moment budget 可阻断连续 surprise。
- [x] reduced motion 阻断明显 surprise。
- [x] DND / 用户交互 gate 由 orchestrator 统一保护。

## 剩余风险

- 低概率体验在测试中使用确定性时间窗口覆盖；真实长时间桌面挂机只做启动 smoke，未长跑观察。
