# 0278 · 验证记录

## 状态

已实现并通过自动化验证。

## 实现结果

- 新增 `NoveltyPolicy`，记录最近 N 个低敏枚举 ID。
- private life Moment 自动选择前会避开最近同 kind + variant 或同 phrase 的候选。
- 诊断触发不进入 Novelty 选择，仍可指定触发任意 Moment。
- 候选全部用尽时 fallback 到第一个候选，避免无动作。

## 命令验证

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml novelty_policy -- --nocapture`：2 passed。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml private_life -- --nocapture`：4 passed，包含连续 3 次 private life Moment 不直接重复。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：187 passed。
- `PATH=/opt/homebrew/bin:/Users/bytedance/Git/ian/node_modules/.bin:$PATH vitest run`（`apps/desktop`）：122 passed。
- `PATH=/opt/homebrew/bin:/Users/bytedance/Git/ian/node_modules/.bin:$PATH tsc --noEmit`（`apps/desktop`）：通过。
- `PATH=/opt/homebrew/bin:/Users/bytedance/Git/ian/node_modules/.bin:$PATH vite build`（`apps/desktop`）：通过。

## 桌面验证

- 已启动真实 Tauri desktop shell，并截图确认 Ian 可见：`/tmp/ian-0277-0278-desktop.png`。
- 当前 macOS `System Events` Accessibility 为 `false`，无法自动连续点击桌面诊断按钮观察回放。
- 替代验证：Rust Core 单测覆盖重复避免、候选不足 fallback、低敏 ID 校验；前端 fallback 测试覆盖 private life debug action 回放。

## 环境说明

- Codex 内置 Node `v24.14.0` 加载 Rollup 原生包时触发 macOS library validation 签名错误；验证时改用本机 `/opt/homebrew/bin/node v22.22.0` 执行前端命令。
