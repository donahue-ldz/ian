# 0091 · 验证记录

## 2026-05-21

### 自动化验证

- `PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:test`：通过，12 个测试文件 / 84 个测试。
- `PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:typecheck`：通过。
- `PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:build`：通过。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：通过，119 个 Rust 测试。

### 桌面 / 预览验收

- 真实 Tauri shell 启动 smoke 通过，终端未出现 panic。
- 资源包合同测试覆盖 `ian-adventurer` sprite 标记、frame capacity、idle/run/zoomies 帧数量和语义动画完整性。

### 剩余风险

- 未在真实桌面上逐帧目视确认点击命中区和气泡头顶位置；后续验收应补一次人工视觉检查。
