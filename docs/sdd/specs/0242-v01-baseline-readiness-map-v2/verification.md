# 0242 · 验证记录

本 SDD 已完成实现和验证。

## 计划验证项

- SDD packet 完整。
- 相关测试或文档断言覆盖本 SDD 验收标准。
- 不引入 P0 禁止能力。
- 真实 Tauri 桌面壳回归验收。

## 计划命令

- `PATH=/opt/homebrew/bin:$PATH npm run test --workspace @ian/desktop -- <target tests>`
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:test`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:typecheck`
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:build`
- `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml --check`
- `git diff --check`
- 真实 Tauri 桌面壳验收。

## 实际结果

- RED：`PATH=/opt/homebrew/bin:$PATH npm run test --workspace @ian/desktop -- src/state/ianActions.test.ts src/renderer/Bubble.test.ts src/lib/tauriBridge.browserFallback.test.ts src/acceptance/post0231ReadinessDocs.test.ts`：失败 8 项，命中新缺口：临时特效过期函数缺失、语义动画行为映射缺失、气泡输入未折叠/截断、browser fallback 未处理 reduced/DND、post-0231 readiness docs/ledger 未更新。
- GREEN 目标测试：同一命令重跑，4 files / 27 tests passed。
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:test`：15 files / 113 tests passed。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：161 Rust tests passed。
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:typecheck`：passed。
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:build`：passed，Vite build completed。
- `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml --check`：passed。
- `git diff --check`：passed before verification document update。
- `PATH=/opt/homebrew/bin:$PATH npm run tauri --workspace @ian/desktop -- dev --config '{"build":{"beforeDevCommand":""}}' --no-dev-server-wait`：Tauri dev shell built and launched `target/debug/ian_desktop`; process stopped after launch verification。

## 剩余风险

- 本批桌面验收确认真实壳可启动；未逐项人工点击 Post-0231 smoke checklist。
- 当前工作树仍包含 0192-0251 连续未提交改动；本轮未自动提交。
- Future capabilities 仍必须保持 default-off / not product-ready，后续 SDD 不得把 readiness 文档误解为产品启用。
