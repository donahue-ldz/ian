# 0102 · 验证记录

## 2026-05-21

### 自动化验证

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：通过，覆盖损坏 TOML 恢复默认配置、旧配置缺字段默认值、持久化 round-trip。
- `desktop:test` / `desktop:typecheck` / `desktop:build`：通过。

### 桌面 / 预览验收

- 真实 Tauri shell smoke 启动成功。

### 剩余风险

- 未在真实用户配置目录写入损坏 config 做 destructive 风格手工测试，避免影响本机现有配置。
