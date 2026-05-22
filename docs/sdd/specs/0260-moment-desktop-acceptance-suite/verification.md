# 0260 · 验证记录

## 2026-05-22

### RED

- `npm run test --workspace @ian/desktop -- src/acceptance/momentDesktopAcceptance.test.ts`
  - 结果：失败，符合预期。缺少 `docs/sdd/moment-desktop-acceptance.md` 验收清单。

### GREEN

- `npm run test --workspace @ian/desktop -- src/acceptance/momentDesktopAcceptance.test.ts`
  - 结果：通过。验收清单覆盖 8 个 moment / gate 章节，每章包含 Pass / Fail condition。
- `npm run desktop:test`
  - 结果：通过。16 个测试文件 / 114 个前端测试通过。
- `npm run desktop:typecheck`
  - 结果：通过。
- `npm run desktop:build`
  - 结果：通过。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 结果：通过。169 个 Rust 测试通过。
- `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml --check`
  - 结果：通过。
- `git diff --check`
  - 结果：通过。
- `npm run tauri --workspace @ian/desktop -- dev --config '{"build":{"beforeDevCommand":""}}' --no-dev-server-wait`
  - 结果：通过。真实 Tauri 桌面壳编译并启动到 `target/debug/ian_desktop`，随后关闭且无残留进程。

## 验收标准结果

- [x] `docs/sdd/moment-desktop-acceptance.md` 覆盖 Find Ian Entrance、Pointer Curiosity、Drag Carry、Drop Settle、Rare Idle Surprise、Memory Echo、Cooldown And Budget、Reduced Motion And DND。
- [x] 每个章节都有明确 Pass condition 和 Fail condition。
- [x] 文档明确真实 Tauri 桌面壳是验收口径。
- [x] 自动测试保护验收清单不被误删或弱化。

## 剩余风险

- 已完成真实 Tauri 启动 smoke；未在本轮逐项人工操作所有桌面 moment。后续视觉调参时应按 `docs/sdd/moment-desktop-acceptance.md` 做人工验收。
