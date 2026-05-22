# 0268 · 实施计划

## 步骤

1. 基于 0262 记录确认气泡方正、遮挡或文案不自然的问题。
2. 梳理 Moment 气泡文案来源和显示时机。
3. 调整气泡视觉样式、定位规则和时长。
4. 优化 Moment 文案，保持短句和角色感。
5. 补充测试并真实桌面验收。

## 预计改动文件

- `apps/desktop/src/renderer/Bubble.tsx`
- `apps/desktop/src/renderer/bubbleModel.ts`
- `apps/desktop/src/renderer/ianStage.css`
- `apps/desktop/src-tauri/src/domain/dialogue/providers/demo.rs`
- `apps/desktop/src-tauri/src/domain/behavior/moment_orchestrator.rs`
- 相关测试文件
- `docs/sdd/specs/0268-bubble-copy-and-visual-delight-polish/verification.md`

## 受影响接口或模块

- Bubble renderer
- Moment speech action
- Demo dialogue copy

## 兼容性说明

样式调整不改变持久化数据；协议字段如新增必须默认兼容。

## 验证命令或手动检查

- `npm run desktop:test`
- `npm run desktop:typecheck`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- 真实 Tauri 桌面观察 Moment 气泡。

## 风险和回滚

- 风险：气泡过弱导致反馈不明显。
- 缓解：以真实桌面截图或观察记录调参。
- 回滚：恢复旧样式和文案集合。
