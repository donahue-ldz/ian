# 0097 · 验证记录

## 2026-05-21

### 自动化验证

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：通过，覆盖 idle 微动作冷却、用户交互抢占、reduced disturbance / quiet 分支。
- `desktop:test` / `desktop:typecheck` / `desktop:build`：通过。

### 桌面 / 预览验收

- 真实 Tauri shell smoke 启动成功；未等待完整 2 分钟 idle 观察。

### 剩余风险

- 需补一次真实桌面长时间 idle 观察，确认微动作节奏主观舒适。
