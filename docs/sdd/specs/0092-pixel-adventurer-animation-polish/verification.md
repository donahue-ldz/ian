# 0092 · 验证记录

## 2026-05-21

### 自动化验证

- `PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:test`：通过，12 个测试文件 / 84 个测试。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：通过，119 个 Rust 测试。

### 桌面 / 预览验收

- 资源包测试确认所有内置 pack 包含 7 个语义动画，`ian-adventurer` 的 idle / run / zoomies 帧序列可区分且在 sprite sheet frame capacity 内。
- 真实 Tauri shell 启动 smoke 通过。

### 剩余风险

- 未录屏或截图比较真实桌面动画节奏；当前以资源合同测试和启动 smoke 代替。
