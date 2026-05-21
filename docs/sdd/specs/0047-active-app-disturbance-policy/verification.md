# 验证记录: Active App Disturbance Policy

## 状态

已实现，待用户验收。

## 实际验证

- [x] `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - Rust: 76 个 tests 通过。
- [x] BehaviorPolicy / ReminderPolicy tests 覆盖 meeting / presentation / focus 降打扰。
- [x] Active app presence 事件只允许 category / confidence / app_id。
- [x] Protocol test 覆盖 `window_title` 等敏感额外字段拒绝。

## 剩余风险

- 未接真实活动应用识别；当前只做 category payload 和 policy 边界。
