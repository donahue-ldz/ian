# 0274 · 实施计划

## 步骤

1. 依赖 0271-0272 或确认已有 story / motion 能力。
2. 检查现有 Find Ian command、tray、shortcut 和 safe zone。
3. 设计 peek target、enter target、final settle target。
4. 输出 find Ian story action sequence。
5. 增加连续触发、safe zone、reduced motion 测试。
6. 真实桌面从不同位置触发找回并记录结果。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/moment_orchestrator.rs`
- `apps/desktop/src/lib/position.ts`
- `apps/desktop/src/renderer/IanStage.tsx`
- `apps/desktop/src/renderer/ianStage.css`
- `apps/desktop/src/state/ianActions.ts`
- `docs/sdd/specs/0274-find-ian-peek-and-return-story/verification.md`

## 受影响接口或模块

- Find Ian Moment
- safe zone position
- story / movement executor

## 兼容性说明

保留现有找回入口，不改变用户触发方式。

## 验证命令或手动检查

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm run desktop:test`
- `npm run desktop:typecheck`
- 真实 Tauri 桌面找回验收。

## 风险和回滚

- 风险：入场 story 太慢影响找回效率。
- 缓解：总时长控制在 3-5 秒，点击 / 拖动可打断。
- 回滚：恢复直接找回到 safe zone。
