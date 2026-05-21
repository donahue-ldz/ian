# 实现计划: Desktop Movement Execution

## 对应规格

`docs/sdd/specs/0021-desktop-movement-execution/spec.md`

## 实现步骤

1. 为前端 movement reducer / executor 写失败测试，覆盖 `movement.move_to` 被记录并执行。
2. 在 `apps/desktop/src/state` 或现有 action hook 中增加 movement 执行状态。
3. 在 Tauri bridge 中增加窗口移动执行函数，浏览器 fallback 使用 transform 目标状态。
4. 在 `IanStage` 或外层容器接入 movement execution，不改变行为决策。
5. 补充 smoke 验证并更新 `verification.md`。

## 预计改动文件

- `apps/desktop/src/state/ianActions.ts`
- `apps/desktop/src/state/ianActions.test.ts`
- `apps/desktop/src/lib/tauriBridge.ts`
- `apps/desktop/src/renderer/IanStage.tsx`
- `apps/desktop/src/renderer/ianStage.css`
- `docs/sdd/specs/0021-desktop-movement-execution/*`

## 接口 / 兼容性

复用 `IanAction::MovementMoveTo`，不引入新外部权限。浏览器 fallback 不影响 Tauri 行为。

## 验证命令

```bash
npm run desktop:test
npm run desktop:typecheck
npm run desktop:build
```

必要时补充 Tauri dev smoke。

## 风险和回滚

窗口移动在不同 macOS 显示器和缩放设置下可能有坐标差异。回滚方式是保留 reducer 测试，临时关闭 Tauri window move executor，只保留浏览器 transform fallback。
