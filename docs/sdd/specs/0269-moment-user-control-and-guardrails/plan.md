# 0269 · 实施计划

## 步骤

1. 梳理现有设置、DND、reduced motion 和 Moment 预算实现。
2. 设计最小用户控制字段和默认值。
3. 在 Rust Core 统一 guardrail 中接入控制项。
4. 在设置或诊断界面展示控制和抑制原因。
5. 对所有 Moment 做模式验收。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/moment_orchestrator.rs`
- `apps/desktop/src-tauri/src/app/runtime.rs`
- `apps/desktop/src/renderer/SettingsPanel.tsx`
- `apps/desktop/src/renderer/settingsModel.ts`
- `apps/desktop/src/state/ianActions.ts`
- 相关测试文件
- `docs/sdd/specs/0269-moment-user-control-and-guardrails/verification.md`

## 受影响接口或模块

- Moment guardrail / policy
- settings model
- diagnostics state
- Tauri config bridge

## 兼容性说明

新增配置必须提供默认值；旧配置缺字段时按保守默认处理。

## 验证命令或手动检查

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm run desktop:test`
- `npm run desktop:typecheck`
- 真实 Tauri 桌面在正常、quiet、reduced motion 模式验收。

## 风险和回滚

- 风险：控制项过多让设置复杂。
- 缓解：只提供最小必要控制，复杂偏好留到后续。
- 回滚：保留 Core 默认策略，隐藏新增设置入口。
