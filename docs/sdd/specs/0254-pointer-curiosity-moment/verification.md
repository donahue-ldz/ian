# 0254 · 验证记录

## 2026-05-22

### RED

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml moment`
  - 结果：失败，符合预期。鼠标靠近只返回基础 attention 反应，缺少 pointer curiosity moment 的低频诊断和上下文 gate。

### GREEN

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml moment`
  - 结果：通过。覆盖 `pointer_curiosity_moment_is_low_frequency_and_context_gated`。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 结果：通过。169 个 Rust 测试通过。
- `npm run desktop:test`
  - 结果：通过。16 个测试文件 / 114 个前端测试通过。
- `npm run desktop:typecheck`
  - 结果：通过。
- `npm run desktop:build`
  - 结果：通过。

## 验收标准结果

- [x] 鼠标靠近 Ian 窗口可产生短 attention + `wave` moment。
- [x] 用户正在交互时阻断 curiosity moment。
- [x] 冷却内不重复触发。
- [x] 低频阻断以 `PlayfulDiagnostic` 记录，不暴露敏感上下文。

## 剩余风险

- 桌面壳已 smoke；未使用真实鼠标逐项录屏观察 hover 动画。
