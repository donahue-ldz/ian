# 0276 · 实施计划

## 步骤

1. 依赖 0275 或确认拖动稳定。
2. 检查当前 drop settle action 和位置持久化时机。
3. 设计 settle story：land、comfort_check、idle_return。
4. 将 Life Drive comfort / energy 纳入 settle 变体选择。
5. 增加连续拖放、reduced motion 和位置持久化测试。
6. 真实桌面拖放并观察收尾。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/moment_orchestrator.rs`
- `apps/desktop/src-tauri/src/domain/behavior/life_drive.rs`
- `apps/desktop/src/renderer/IanStage.tsx`
- `apps/desktop/src/renderer/ianStage.css`
- `apps/desktop/src/lib/position.test.ts`
- `docs/sdd/specs/0276-drop-comfort-settle-v2/verification.md`

## 受影响接口或模块

- Drop Settle Moment
- position persistence
- Life Drive comfort

## 兼容性说明

不改变持久化位置格式；settle 只影响视觉和短状态。

## 验证命令或手动检查

- `npm run desktop:test`
- `npm run desktop:typecheck`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- 真实 Tauri 桌面拖放验收。

## 风险和回滚

- 风险：settle 太长影响用户继续拖动。
- 缓解：用户输入立即打断。
- 回滚：drop 后直接 idle。
