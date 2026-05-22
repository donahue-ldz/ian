# 0193 · 验证记录

待实现后更新。

## 计划验证项

- 桌面端拖动期间不再同时执行手动 setPosition 位移。
- 拖动结束保存 Tauri outerPosition。
- 浏览器预览拖动仍可用。
- 真实 Tauri 桌面端三屏拖动验收记录完整。

## 计划命令

- Rust 目标测试：`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- 前端目标测试：`PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:test`
- 类型检查：`PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:typecheck`
- 涉及桌面可见行为时：真实 Tauri 桌面端验收。

## 实际结果

待实现后记录实际命令、结果、失败项和跳过项。

## 剩余风险

实现前无验证结果。
