# Verification: Desktop Window Move Permission

## Spec

`docs/sdd/specs/0085-desktop-window-move-permission/spec.md`

## Verification Summary

已完成 TDD RED/GREEN 验证。缺少 `core:window:allow-set-position` 时 capability 回归测试失败；补齐权限后同一测试通过，`movement.move_to` 状态执行链路测试也通过。

## Checks

| Check | Command / Method | Result | Notes |
| --- | --- | --- | --- |
| RED capability regression | `npm run desktop:test -- IanStage.test.tsx` | Failed as expected | 1 failed / 11 passed；失败原因是 `capability.permissions` 缺少 `core:window:allow-set-position`。 |
| GREEN capability regression | `npm run desktop:test -- IanStage.test.tsx` | Passed | 1 file / 13 tests passed。 |
| Movement action state regression | `npm run desktop:test -- ianActions.test.ts` | Passed | 1 file / 12 tests passed。 |
| Frontend typecheck | `npm run desktop:typecheck` | Passed | `tsc --noEmit` 退出码为 0。 |

## Acceptance Criteria Results

- [x] `apps/desktop/src-tauri/capabilities/default.json` 包含 `core:window:allow-set-position`。
- [x] capability 回归测试在缺少 `core:window:allow-set-position` 时失败。
- [x] `movement.move_to` 的执行链路仍通过 `moveDesktopWindow()` 调用 Tauri window API，不在 React 中生成行为目标。
- [x] 相关前端测试通过。

## Failures or Gaps

未执行 Tauri dev 实机 smoke；本次验证覆盖权限配置和 action 执行状态链路，但未实际观察 macOS 原生窗口坐标变化。

## Follow-Ups

- 增加 Tauri dev 实机 smoke：触发一次 `movement.move_to` 后记录 `outerPosition()` 变化。
