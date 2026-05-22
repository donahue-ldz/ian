# 0264 · 验证记录

## 2026-05-22

### 已执行检查

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 结果：通过。171 个 Rust 测试通过，覆盖找回、safe target、连续触发冷却和 reduced motion 相关路径。
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:test`
  - 结果：通过。16 个测试文件 / 117 个测试通过。
- 真实 Tauri 桌面壳启动 smoke
  - 结果：通过。启动到 `target/debug/ian_desktop` 后关闭。

## 验收标准结果

- [x] 不可见位置可通过找回路径移动回安全区域。
- [x] 找回包含短 beacon、短气泡和 `find`/`happy` 反馈。
- [x] 连续触发由 cooldown 阻止重复移动。
- [x] reduced motion 下诊断找回可降级，避免主动大幅移动。
- [x] 找回只使用屏幕几何和 Ian 状态，不读取屏幕内容。

## 剩余风险

- 本机未执行多屏人工验收；多屏实际落点仍需在可用环境补验。
