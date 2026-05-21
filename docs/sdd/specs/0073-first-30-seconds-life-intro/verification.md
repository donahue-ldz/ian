# 0073 验证记录

## 2026-05-21

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml domain::behavior::behavior_engine`：通过，23 个行为引擎测试通过。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：通过，91 个 Rust 测试通过。
- `npm run desktop:test`：通过，11 个测试文件、50 个测试通过。
- `npm run desktop:typecheck`：通过。
- `npm run desktop:build`：通过。
- `git diff --check`：通过。
- `curl -I http://127.0.0.1:1420/`：通过，Vite dev server 返回 200。
- 浏览器冒烟检查：通过，页面标题为 `Ian`，存在 Ian 舞台和 Ian 按钮。

## 验收结论

通过。`app.started` 会输出短促醒来反馈，并且不产生位置移动动作。
