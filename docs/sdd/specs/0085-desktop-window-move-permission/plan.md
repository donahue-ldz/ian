# Implementation Plan: Desktop Window Move Permission

## Spec

`docs/sdd/specs/0085-desktop-window-move-permission/spec.md`

## Summary

补齐 Tauri 桌面窗口移动权限，并添加回归测试直接检查 capability 配置，覆盖浏览器预览无法证明的桌面真实移动边界。

## Steps

1. 在 `apps/desktop/src/renderer/IanStage.test.tsx` 中先添加失败测试，读取 `apps/desktop/src-tauri/capabilities/default.json` 并断言包含 `core:window:allow-set-position`。
2. 运行 `npm run desktop:test -- IanStage.test.tsx`，确认测试因缺少 `core:window:allow-set-position` 失败。
3. 在 `apps/desktop/src-tauri/capabilities/default.json` 的 `permissions` 中加入 `core:window:allow-set-position`。
4. 重新运行 `npm run desktop:test -- IanStage.test.tsx`，确认回归测试通过。
5. 运行 `npm run desktop:test -- ianActions.test.ts`，确认 `movement.move_to` action 执行状态链路未回退。
6. 更新 `verification.md` 记录 RED/GREEN 和验证结果。

## Expected File Changes

- `apps/desktop/src/renderer/IanStage.test.tsx`: 增加桌面窗口移动 capability 回归测试。
- `apps/desktop/src-tauri/capabilities/default.json`: 增加 `core:window:allow-set-position`。
- `docs/sdd/specs/0085-desktop-window-move-permission/*`: 记录规格、计划、决策和验证。

## Interfaces and Boundaries

不改变协议、Rust Core 行为策略或 React action 执行接口。该修复只补齐 Tauri 权限边界，让现有 `movement.move_to` 到 `moveDesktopWindow()` 的桌面执行链路可用。

## Verification Commands

```bash
npm run desktop:test -- IanStage.test.tsx
npm run desktop:test -- ianActions.test.ts
```

## Risks

- 风险：Tauri capability 测试只能证明配置存在，不能完全替代桌面实机观察。
- 缓解：本次先补齐确定缺失的授权；后续可增加 Tauri dev 实机 smoke，观察窗口坐标在 `movement.move_to` 后变化。

## Rollback Notes

如需回滚，移除 `core:window:allow-set-position` 和对应测试即可；自动游走行为决策仍会保留，但桌面真实窗口移动会再次失去权限。
