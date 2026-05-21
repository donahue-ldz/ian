# 实现计划: Core Life Telemetry Local

## 对应规格

`docs/sdd/specs/0039-core-life-telemetry-local/spec.md`

## 实现步骤

1. 为 decision summary 写 Rust 失败测试。
2. 增加 diagnostics model 和 allowlist。
3. 在 BehaviorEngine / LifeRhythmPolicy 输出决策摘要。
4. 增加保留期或关闭配置。
5. 做 source scan 和本地 smoke。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/behavior_engine.rs`
- `apps/desktop/src-tauri/src/domain/behavior/life_rhythm_policy.rs`
- `apps/desktop/src-tauri/src/storage/life_event_repository.rs`
- `apps/desktop/src-tauri/src/security/sanitizer.rs`
- `docs/sdd/specs/0039-core-life-telemetry-local/*`

## 接口 / 兼容性

诊断默认本地，不能引入网络依赖。缺少 repository 时行为照常执行。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml storage
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml security
```

## 风险和回滚

诊断日志可能膨胀或过度记录。回滚方式是仅保留测试中的 in-memory diagnostics，不落盘。
