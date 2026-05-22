# 0252 · 验证记录

## 2026-05-22

### RED

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml moment`
  - 结果：失败，符合预期。缺少 moment 编排层、per-kind cooldown、全局预算和 memory echo 过滤能力。

### GREEN

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml moment`
  - 结果：通过。9 个 moment 相关 Rust 测试通过。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 结果：通过。169 个 Rust 测试通过。
- `npm run desktop:test`
  - 结果：通过。16 个测试文件 / 114 个前端测试通过。
- `npm run desktop:typecheck`
  - 结果：通过。
- `npm run desktop:build`
  - 结果：通过。
- `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml --check`
  - 结果：通过。
- `git diff --check`
  - 结果：通过。
- `npm run tauri --workspace @ian/desktop -- dev --config '{"build":{"beforeDevCommand":""}}' --no-dev-server-wait`
  - 结果：通过。真实 Tauri 桌面壳编译并启动到 `target/debug/ian_desktop`，随后关闭且无残留进程。

## 验收标准结果

- [x] Rust Core 提供 `MomentOrchestrator`，按 moment kind 做冷却。
- [x] 全局 budget 限制连续惊喜触发。
- [x] DND、reduced motion 和用户交互状态可阻断 moment。
- [x] 输出仍为 `IanAction`，React 不维护核心 moment 状态。
- [x] 诊断动作只包含低敏 reason、kind 和时间。

## 剩余风险

- 桌面壳验证为启动 smoke，没有逐项人工观察所有 moment 动画；具体桌面验收清单见 `0260`。
