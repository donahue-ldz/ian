# 0269 · 验证记录

## 2026-05-22

### 已执行检查

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml moment_guardrails_explain_user_control_blocks`
  - 结果：通过。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 结果：通过。171 个 Rust 测试通过。
- `PATH=/opt/homebrew/bin:$PATH npm run test --workspace @ian/desktop -- src/renderer/SettingsPanel.view.test.tsx`
  - 结果：通过。
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:typecheck`
  - 结果：通过。

## 验收标准结果

- [x] 设置中已有移动强度、气泡频率、勿扰、本地诊断和 playful energy 控制。
- [x] Core guardrail 对非必要 Moment 统一检查 `playful_energy=off` 和 `behavior_mode=quiet`。
- [x] DND / reduced motion 继续阻断或降级主动 Moment。
- [x] 诊断状态通过 `PlayfulDiagnostic` 返回 `blocked_user_control`、`blocked_dnd`、`blocked_reduced_motion` 等低敏 reason。
- [x] React 只展示设置和发送配置 / 诊断事件，不维护核心 Moment 策略。

## 剩余风险

- 未在真实桌面逐项切换正常、quiet、reduced motion 三种模式人工观察；自动测试覆盖策略路径。
