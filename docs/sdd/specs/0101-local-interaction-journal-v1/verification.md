# 0101 · 验证记录

## 2026-05-21

### 自动化验证

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：通过，覆盖 `life_events_append_and_read_in_time_order`、低敏 payload allowlist、interaction summary 不保存正文。
- `desktop:test` / `desktop:typecheck` / `desktop:build`：通过。

### 桌面 / 预览验收

- 真实 Tauri shell smoke 启动成功；未直接打开 SQLite 人工检查新增事件。

### 剩余风险

- 后续可补一条桌面点击后检查本地 SQLite life_events 的手动记录。
