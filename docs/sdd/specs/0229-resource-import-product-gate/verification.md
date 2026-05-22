# 0229 · 验证记录

本 SDD 已完成实现和验证。

## 计划验证项

- 门禁列出安全、尺寸、manifest、回滚要求。
- 明确当前是否允许产品化。

## 计划命令

- Rust 目标测试：`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- 前端目标测试：`PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:test`
- 类型检查：`PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:typecheck`
- 涉及桌面可见行为时：真实 Tauri 桌面端验收。

## 实际结果

- `PATH=/opt/homebrew/bin:$PATH npm run test --workspace @ian/desktop -- src/resources/resourceLoader.test.ts src/renderer/Bubble.view.test.tsx src/renderer/IanStage.test.tsx src/renderer/settingsModel.test.ts src/acceptance/platformReadinessDocs.test.ts`：5 files / 46 tests passed。
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:test`：13 files / 105 tests passed。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：161 Rust tests passed。
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:typecheck`：passed。
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:build`：passed，Vite build completed。
- `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml --check`：passed。
- `git diff --check`：passed before this verification document update。
- `PATH=/opt/homebrew/bin:$PATH npm run tauri --workspace @ian/desktop -- dev --config '{"build":{"beforeDevCommand":""}}' --no-dev-server-wait`：Tauri dev shell built and launched `target/debug/ian_desktop`; process stopped after launch verification。

## 剩余风险

- 桌面验收确认真实壳可启动；未在本 SDD 中新增人工视觉截图审查。
- 0232-0241 SDD 目录不存在，未执行也未创建占位。
