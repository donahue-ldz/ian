# 0278 · 实施计划

## 步骤

1. 查看现有 Moment kind、animation variant、bubble phrase 表示方式。
2. 新增短期 Novelty History，记录低敏枚举 ID。
3. 在 Moment / phrase 选择前应用重复降权。
4. 保留诊断模式指定触发，不受 Novelty 干扰。
5. 增加重复避免、候选不足、隐私字段测试。
6. 真实桌面连续触发几个 Moment，观察文案和动作不机械重复。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/novelty_policy.rs`
- `apps/desktop/src-tauri/src/domain/behavior/moment_orchestrator.rs`
- `apps/desktop/src-tauri/src/domain/dialogue/providers/demo.rs`
- `apps/desktop/src-tauri/src/domain/dialogue/dialogue_engine.rs`
- `docs/sdd/specs/0278-novelty-and-repeat-avoidance/verification.md`

## 受影响接口或模块

- Moment selection
- Demo dialogue phrase selection
- Behavior policy

## 兼容性说明

短期内存记录不影响持久化。重启后 Novelty History 可重置。

## 验证命令或手动检查

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm run desktop:test`
- 真实 Tauri 桌面连续触发观察。

## 风险和回滚

- 风险：候选过少导致无动作。
- 缓解：候选不足时允许最低优先级重复。
- 回滚：关闭 Novelty Policy。
