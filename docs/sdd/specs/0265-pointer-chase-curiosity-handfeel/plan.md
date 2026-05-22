# 0265 · 实施计划

## 步骤

1. 基于 0262 记录确认当前追随无法触发或手感问题。
2. 检查 pointer 事件上报、Moment 决策和移动执行路径。
3. 增加或调整追随状态、速度、距离、停止条件。
4. 区分诊断 100% 触发和正式冷却触发。
5. 补充测试并真实桌面验收。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/moment_orchestrator.rs`
- `apps/desktop/src/renderer/IanStage.tsx`
- `apps/desktop/src/renderer/ianStage.css`
- `apps/desktop/src/state/ianActions.ts`
- 相关测试文件
- `docs/sdd/specs/0265-pointer-chase-curiosity-handfeel/verification.md`

## 受影响接口或模块

- Pointer Curiosity Moment
- Movement executor
- pointer event bridge

## 兼容性说明

如果新增 pointer 字段，必须保持旧事件默认可用。

## 验证命令或手动检查

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm run desktop:test`
- `npm run desktop:typecheck`
- 真实 Tauri 桌面鼠标靠近 / 离开 / 点击验收。

## 风险和回滚

- 风险：追随太频繁影响工作。
- 缓解：正式模式保留冷却、预算和停止条件。
- 回滚：关闭追随 Moment，仅保留轻微好奇反馈。
