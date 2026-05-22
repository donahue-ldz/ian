# 0247 · 实现计划

## 有序步骤

1. 读取当前阶段、架构规则和相关代码 / 文档。
2. 先写能失败的测试或文档断言，证明当前缺口存在。
3. 按最小范围更新：docs/codex/social-plugin-nonproduct-gate-audit.md。
4. 保持 P0 / v0.1 边界，未来能力默认关闭或只做 readiness 记录。
5. 运行目标测试，再运行全量验证。
6. 启动真实 Tauri 桌面壳做回归验收。
7. 更新 decisions.md 和 verification.md。

## 预计改动文件

- docs/codex/social-plugin-nonproduct-gate-audit.md
- 相关测试文件。
- 本 SDD 的 decisions.md / verification.md。

## 接口 / 模块影响

- 不新增对外网络、全局输入或插件执行入口。
- 如涉及前端状态，只更新 React view state 执行动作逻辑。
- 如涉及文档 readiness，用 acceptance test 锁定关键结论。

## migration / 兼容性

- 默认无 migration。
- 不改变已有 config / SQLite schema。

## 验证命令

- `PATH=/opt/homebrew/bin:$PATH npm run test --workspace @ian/desktop -- <target tests>`
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:test`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:typecheck`
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:build`
- `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml --check`
- `git diff --check`
- `PATH=/opt/homebrew/bin:$PATH npm run tauri --workspace @ian/desktop -- dev --config '{"build":{"beforeDevCommand":""}}' --no-dev-server-wait`

## 风险和回滚

- 若测试显示范围扩大到未来产品能力，回滚该实现并只保留 readiness 文档。
- 若桌面壳无法启动，verification 必须记录原因和剩余风险。
