# 0255 · 验证记录

## 2026-05-22

### RED

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml moment`
  - 结果：失败，符合预期。拖拽开始缺少 carry moment diagnostic、短缩放和独立承载反馈。

### GREEN

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml moment`
  - 结果：通过。覆盖 `drag_carry_and_drop_settle_moments_are_short_and_non_blaming`。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 结果：通过。169 个 Rust 测试通过。
- `npm run desktop:test`
  - 结果：通过。16 个测试文件 / 114 个前端测试通过。
- `npm run desktop:typecheck`
  - 结果：通过。
- `npm run desktop:build`
  - 结果：通过。

## 验收标准结果

- [x] 拖动开始产生 `drag_carry` diagnostic。
- [x] 拖动开始短暂 scale 到 `0.92` 并播放 affection/happy 反馈。
- [x] 拖动期间不引入大量动作堆叠。
- [x] 文案保持轻量，不责备、不索取、不把用户绑定成照顾者。

## 剩余风险

- 自动测试覆盖 Rust action 序列；真实拖动手感只完成 Tauri 启动 smoke，未人工逐项验收。
