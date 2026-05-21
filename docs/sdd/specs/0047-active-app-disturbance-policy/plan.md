# 实现计划: Active App Disturbance Policy

## 对应规格

`docs/sdd/specs/0047-active-app-disturbance-policy/spec.md`

## 实现步骤

1. 为 category 降打扰写 behavior / reminder policy 失败测试。
2. 加固 active app adapter payload allowlist。
3. 将 category 接入 Life Rhythm / Reminder 降级策略。
4. 设置面保留显式授权。
5. 做 source scan 和 mock presence smoke。

## 预计改动文件

- `apps/desktop/src-tauri/src/adapters/active_app_presence_adapter.rs`
- `apps/desktop/src-tauri/src/domain/behavior/behavior_policy.rs`
- `apps/desktop/src-tauri/src/domain/reminder/reminder_policy.rs`
- `apps/desktop/src-tauri/src/security/sanitizer.rs`
- `docs/sdd/specs/0047-active-app-disturbance-policy/*`

## 接口 / 兼容性

未知 category 默认不改变行为。旧事件 payload 仍可处理，但敏感字段必须被拒绝。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml adapters
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml reminder
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml security
```

## 风险和回滚

活动应用类别可能误判。回滚方式是只在 meeting / presentation 降低提醒，不影响移动。
