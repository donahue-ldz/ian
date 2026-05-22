# 0194 · 验证记录

待实现后更新。

## 计划验证项

- 快捷键默认关闭或由用户明确开启。
- 关闭后不再响应全局快捷键。
- 触发事件不包含键盘文本或节奏数据。
- Ian 越界时回到可见安全区域，已可见时只短回应。
- 真实桌面验收记录完整。

## 计划命令

- Rust 目标测试：`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- 前端目标测试：`PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:test`
- 类型检查：`PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:typecheck`
- 涉及桌面可见行为时：真实 Tauri 桌面端验收。

## 实际结果

待实现后记录实际命令、结果、失败项和跳过项。

## 剩余风险

实现前无验证结果。
