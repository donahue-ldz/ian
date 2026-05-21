# Verification: Smooth Desktop Window Movement

## Spec

`docs/sdd/specs/0086-smooth-desktop-window-movement/spec.md`

## Verification Summary

已完成 RED/GREEN 验证。新增测试先因缺少 `planDesktopMovementFrames` 失败；实现多帧移动计划并接入 `movement.move_to` 后，相关测试和类型检查通过。

## Checks

| Check | Command / Method | Result | Notes |
| --- | --- | --- | --- |
| RED smooth movement regression | `npm run desktop:test -- ianActions.test.ts` | Failed as expected | 1 failed / 12 passed；失败原因是 `planDesktopMovementFrames is not a function`。 |
| GREEN smooth movement regression | `npm run desktop:test -- ianActions.test.ts` | Passed | 1 file / 13 tests passed。 |
| Frontend typecheck | `npm run desktop:typecheck` | Passed | `tsc --noEmit` 退出码为 0。 |
| Desktop capability regression | `npm run desktop:test -- IanStage.test.tsx` | Passed | 1 file / 13 tests passed。 |

## Acceptance Criteria Results

- [x] 桌面 movement 执行计划包含多帧中间坐标，最后一帧等于目标位置。
- [x] `fast`、`normal`、`slow` 的移动时长保持可区分，且 `slow > normal > fast`。
- [x] `movement.move_to` 执行链路使用平滑移动 helper，而不是单次 `moveDesktopWindow(target)`。
- [x] 相关前端测试和类型检查通过。

## Failures or Gaps

未执行 Tauri dev 实机 smoke；本次验证覆盖移动计划、action 执行代码路径、类型检查和桌面 capability 回归。

## Follow-Ups

- 用 `npm run desktop:tauri -- dev` 观察真实桌面窗口移动速度；如仍偏快，可继续调 `durationForSpeed()` 的三个档位。
