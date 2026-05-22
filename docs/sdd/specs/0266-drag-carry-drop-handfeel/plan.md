# 0266 · 实施计划

## 步骤

1. 基于 0262 记录定位拖不动、偏移或放下反馈问题。
2. 检查 IanStage 命中区、pointer capture、窗口移动实现。
3. 调整拖动中状态和放下 Moment action。
4. 确认拖动期间抑制其他 Moment。
5. 验证位置持久化和真实桌面手感。

## 预计改动文件

- `apps/desktop/src/renderer/IanStage.tsx`
- `apps/desktop/src/renderer/ianStage.css`
- `apps/desktop/src/lib/position.ts`
- `apps/desktop/src-tauri/src/domain/behavior/moment_orchestrator.rs`
- 相关测试文件
- `docs/sdd/specs/0266-drag-carry-drop-handfeel/verification.md`

## 受影响接口或模块

- Drag Carry Moment
- Drop Settle Moment
- position persistence
- pointer input handling

## 兼容性说明

保持既有位置配置兼容；不迁移用户数据。

## 验证命令或手动检查

- `npm run desktop:test`
- `npm run desktop:typecheck`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- 真实 Tauri 桌面拖动 / 放下 / 重启验收。

## 风险和回滚

- 风险：拖动命中区过大影响点击气泡。
- 缓解：区分点击、双击、拖动阈值。
- 回滚：恢复旧拖动逻辑，保留测试记录。
