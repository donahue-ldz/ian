# Implementation Plan: Smooth Desktop Window Movement

## Spec

`docs/sdd/specs/0086-smooth-desktop-window-movement/spec.md`

## Summary

在前端 action 执行层增加桌面窗口移动插值计划。`movement.move_to` 先读取当前桌面窗口位置，再按 speed 对应时长生成多帧坐标，逐帧调用 Tauri `setPosition`。

## Steps

1. 在 `apps/desktop/src/state/ianActions.test.ts` 添加失败测试，断言移动计划有多帧中间坐标、最后一帧是目标点，且 `slow > normal > fast`。
2. 运行 `npm run desktop:test -- ianActions.test.ts`，确认测试因缺少 smooth movement helper 失败。
3. 在 `apps/desktop/src/state/useIanActions.ts` 实现并导出移动计划 helper。
4. 将 `applyActionSequence()` 从单次 `moveDesktopWindow(target)` 改为调用平滑移动 helper。
5. 运行 `npm run desktop:test -- ianActions.test.ts` 和 `npm run desktop:typecheck`。
6. 更新 `verification.md`。

## Expected File Changes

- `apps/desktop/src/state/useIanActions.ts`: 增加移动插值计划并接入 action 执行。
- `apps/desktop/src/state/ianActions.test.ts`: 增加平滑移动回归测试。
- `docs/sdd/specs/0086-smooth-desktop-window-movement/*`: 记录规格、计划、决策和验证。

## Interfaces and Boundaries

不改变协议和 Rust Core。前端继续只执行 `movement.move_to`，只是在 Tauri 桌面执行层把单次窗口定位拆成多帧窗口定位。

## Verification Commands

```bash
npm run desktop:test -- ianActions.test.ts
npm run desktop:typecheck
```

## Risks

- 分帧调用过多可能让 run-around 太慢；通过 speed 时长保持差异，先使用保守帧数。
- 浏览器预览没有 Tauri 窗口坐标，仍保持现有 state 表现。

## Rollback Notes

如需回滚，恢复 `applyActionSequence()` 中对 `moveDesktopWindow(target)` 的单次调用，并移除新增测试和 helper。
