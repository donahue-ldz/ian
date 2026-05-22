# 0195 · 验证记录

待实现后更新。

## 计划验证项

- checklist 覆盖启动、拖动、找回、气泡、设置、资源包。
- 每个步骤有通过/失败判定。
- 明确浏览器验证不能替代桌面验收。
- 不要求读取屏幕内容或用户隐私。

## 计划命令

- Rust 目标测试：`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- 前端目标测试：`PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:test`
- 类型检查：`PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:typecheck`
- 涉及桌面可见行为时：真实 Tauri 桌面端验收。

## 实际结果

待实现后记录实际命令、结果、失败项和跳过项。

## 剩余风险

实现前无验证结果。
