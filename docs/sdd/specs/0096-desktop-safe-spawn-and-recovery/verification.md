# 0096 · 验证记录

## 2026-05-21

### 自动化验证

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：通过，包含 `saved_position_is_clamped_to_visible_monitor_bounds`、`missing_saved_position_uses_bottom_right_default`。
- `desktop:test` / `desktop:typecheck` / `desktop:build`：通过。

### 桌面 / 预览验收

- 真实 Tauri shell smoke 启动成功；新增位置重置命令复用 Tauri 当前 monitor / window outer size 计算右下安全位置。

### 剩余风险

- 未实际写入越界 config 后重启桌面壳做人工观察；风险记录为需补验。
