# 0074 验证记录

## 2026-05-21

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml domain::behavior::behavior_engine`：通过，23 个行为引擎测试通过。
- `npm run desktop:test -- IanStage.test.tsx`：通过，6 个 IanStage 测试通过。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：通过，91 个 Rust 测试通过。
- `npm run desktop:test`：通过，11 个测试文件、50 个测试通过。
- `npm run desktop:typecheck`：通过。
- `npm run desktop:build`：通过。
- `git diff --check`：通过。
- `curl -I http://127.0.0.1:1420/`：通过，Vite dev server 返回 200。
- 浏览器冒烟检查：通过，页面标题为 `Ian`，存在 Ian 舞台和 Ian 按钮，默认 `data-dragging="false"`。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml desktop::window`：通过，3 个窗口定位测试通过，包含越界保存位置夹取。
- `npm run desktop:test -- position.test.ts IanStage.test.tsx`：通过，2 个测试文件、10 个测试通过。
- `npm run desktop:typecheck`：通过。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：通过，92 个 Rust 测试通过。
- `npm run desktop:test`：通过，11 个测试文件、52 个测试通过。
- `npm run desktop:build`：通过。
- `git diff --check`：通过。

## 验收结论

通过。拖动开始、拖动过程和拖动结束都有可区分反馈；桌面模式下拖动结束使用真实窗口坐标，启动时越界保存位置会被夹到当前显示器可见范围内。
