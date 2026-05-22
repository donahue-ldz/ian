# 0267 · 实施计划

## 步骤

1. 基于 0262 记录确认 idle surprise 的缺失、过弱或过强问题。
2. 梳理当前 idle surprise 变体和触发条件。
3. 调整频率、冷却、预算、持续时间和退出条件。
4. 补充 DND / reduced motion / 输入中抑制测试。
5. 做真实桌面诊断触发和 long-run 观察。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/moment_orchestrator.rs`
- `apps/desktop/public/resources/pets/ian-adventurer/animations.json`
- `apps/desktop/src/renderer/IanStage.tsx`
- 相关测试文件
- `docs/sdd/specs/0267-rare-idle-surprise-tuning/verification.md`

## 受影响接口或模块

- Rare Idle Surprise Moment
- animation resource pack
- reduced motion / DND gates

## 兼容性说明

不改变已有用户配置；若增加 surprise 强度配置，必须默认保守。

## 验证命令或手动检查

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm run desktop:test`
- `npm run desktop:typecheck`
- 真实 Tauri 桌面 10-20 分钟观察或使用缩短冷却的诊断模式。

## 风险和回滚

- 风险：低频行为难以人工观察。
- 缓解：诊断触发与正式低频策略分离。
- 回滚：降低 idle surprise 预算或关闭该 Moment。
