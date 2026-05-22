# 0253 · 验证记录

## 2026-05-22

### RED

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml moment`
  - 结果：失败，符合预期。找回入口缺少 moment diagnostic、可见时原地回应和连续触发冷却行为。

### GREEN

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml moment`
  - 结果：通过。覆盖 `find_ian_entrance_moment_moves_only_before_cooldown`。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 结果：通过。169 个 Rust 测试通过。
- `npm run desktop:test`
  - 结果：通过。16 个测试文件 / 114 个前端测试通过。
- `npm run desktop:typecheck`
  - 结果：通过。
- `npm run desktop:build`
  - 结果：通过。
- `npm run tauri --workspace @ian/desktop -- dev --config '{"build":{"beforeDevCommand":""}}' --no-dev-server-wait`
  - 结果：通过。真实 Tauri 桌面壳可启动并关闭。

## 验收标准结果

- [x] `find_ian` 入口触发短视觉 beacon、speech 和 `find` 动画。
- [x] Ian 不可见时会移动回安全可见位置。
- [x] Ian 已可见或处于 cooldown 时不重复移动。
- [x] 连续触发返回低敏 diagnostic，避免堆叠惊喜。
- [x] 不读取窗口内容、屏幕 OCR 或全局键盘文本。

## 剩余风险

- 当前自动验证覆盖 Rust 行为序列；设置入口到同一事件路径的端到端点击只做桌面启动 smoke，未逐项人工点击验收。
