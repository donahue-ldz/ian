# 0264 · 实施计划

## 步骤

1. 基于 0262 记录确认找回失败或体验不足点。
2. 检查当前找回事件、safe zone、窗口移动实现。
3. 调整找回目标位置和入场动作组合。
4. 增加连续触发和 reduced motion 降级处理。
5. 使用 0263 诊断入口和真实桌面验收。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/moment_orchestrator.rs`
- `apps/desktop/src/lib/position.ts`
- `apps/desktop/src/renderer/IanStage.tsx`
- `apps/desktop/src/renderer/ianStage.css`
- 相关测试文件
- `docs/sdd/specs/0264-find-ian-return-and-beacon-tuning/verification.md`

## 受影响接口或模块

- Find Ian Moment
- safe zone / position model
- movement and animation executor

## 兼容性说明

不改变用户已有位置配置格式；如需要新增配置，必须提供默认值和兼容读取。

## 验证命令或手动检查

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm run desktop:test`
- `npm run desktop:typecheck`
- 真实 Tauri 桌面找回 Ian。

## 风险和回滚

- 风险：找回太夸张影响工作。
- 缓解：短时、可打断、reduced motion 降级。
- 回滚：恢复旧找回动作组合。
