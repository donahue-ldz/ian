# 实现计划: Day Night Rhythm

## 对应规格

`docs/sdd/specs/0032-day-night-rhythm/spec.md`

## 实现步骤

1. 为 `DayPhase` 计算写失败测试，覆盖 morning/day/evening/night 边界。
2. 在 Rust Core 增加 day phase policy。
3. 将 day phase 接入 scheduler / behavior policy。
4. 将 day phase 作为 DialogueContext 的低敏字段。
5. 做 source scan，确认没有读取日历、位置或天气。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/day_phase_policy.rs`
- `apps/desktop/src-tauri/src/core/scheduler.rs`
- `apps/desktop/src-tauri/src/domain/dialogue/dialogue_engine.rs`
- `apps/desktop/src-tauri/src/domain/dialogue/dialogue_policy.rs`
- `docs/sdd/specs/0032-day-night-rhythm/*`

## 接口 / 兼容性

如新增 `DayPhase` 到协议或 context，需要提供默认值，避免旧状态加载失败。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml dialogue
```

## 风险和回滚

时间段默认值可能不符合用户习惯。回滚方式是只在 night 降低主动行为，其他时间段暂不区分。
