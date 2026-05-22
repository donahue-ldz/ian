# 0209 · 验证记录

## 实际结果

- PASS: `PATH=/opt/homebrew/bin:$PATH npm run desktop:test`，12 个前端测试文件 / 97 个测试通过。
- PASS: `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`，159 个 Rust 测试通过。
- PASS: `PATH=/opt/homebrew/bin:$PATH npm run desktop:typecheck`。
- PASS: `PATH=/opt/homebrew/bin:$PATH npm run desktop:build`。
- PASS: `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml --check`。
- PASS: `git diff --check`。
- PASS: 真实 Tauri shell 启动：`PATH=/opt/homebrew/bin:$PATH npm run tauri --workspace @ian/desktop -- dev --config '{"build":{"beforeDevCommand":""}}' --no-dev-server-wait` 编译完成并运行 `target/debug/ian_desktop`；随后仅停止本次启动进程。

## 覆盖范围

本轮覆盖 0209-affection-boundary-copy-suite 的代码路径或文档验收项，并随 0202-0221 批次一起完成回归。

## 剩余风险

桌面壳已启动验证，但未人工逐项点击所有设置项；后续若要接受桌面视觉细节，仍建议按 `docs/sdd/desktop-smoke-checklist.md` 做一次手动走查。
