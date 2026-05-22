# 0259 · 验证记录

## 2026-05-22

### RED

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml moment`
  - 结果：失败，符合预期。moment 尚未具备 per-kind cooldown、global budget、DND / reduced motion gate。

### GREEN

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml moment`
  - 结果：通过。覆盖 per-kind cooldown、10 分钟全局 budget、reduced motion gate。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 结果：通过。169 个 Rust 测试通过。
- `npm run desktop:test`
  - 结果：通过。16 个测试文件 / 114 个前端测试通过。
- `npm run desktop:typecheck`
  - 结果：通过。
- `npm run desktop:build`
  - 结果：通过。
- `git diff --check`
  - 结果：通过。

## 验收标准结果

- [x] 每类 moment 有独立 cooldown。
- [x] 全局 surprise budget 限制 10 分钟窗口内的触发次数。
- [x] DND、reduced motion、active interaction 会阻断 surprise。
- [x] diagnostic 只包含 kind、reason、timestamp 等低敏字段。

## 剩余风险

- budget 状态当前在行为引擎内存中维护；跨进程持久化不是本 SDD 范围。
