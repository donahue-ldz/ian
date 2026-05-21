# Verification: Idle Tiny Perimeter Patrol

## Spec

`docs/sdd/specs/0087-idle-tiny-perimeter-patrol/spec.md`

## Verification Summary

已完成 RED/GREEN 验证。Rust 测试先因缺少 `ScreenBounds` / `AppearanceScaleTo` 失败，前端 reducer 测试先因缺少 `appearanceScale` 失败；实现协议、行为和前端执行后，完整 Rust 测试、相关前端测试和类型检查通过。

## Checks

| Check | Command / Method | Result | Notes |
| --- | --- | --- | --- |
| Rust RED behavior | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior` | Failed as expected | 编译失败，缺少 `ScreenBounds`、`screen_bounds` state 字段和 `AppearanceScaleTo` action。 |
| Frontend RED reducer | `npm run desktop:test -- ianActions.test.ts` | Failed as expected | 1 failed / 13 passed；`appearanceScale` 尚未实现。 |
| Rust behavior | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior` | Passed | 51 behavior-related tests passed。 |
| Full Rust tests | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` | Passed | 110 tests passed。 |
| Frontend reducer | `npm run desktop:test -- ianActions.test.ts` | Passed | 1 file / 14 tests passed。 |
| Frontend typecheck | `npm run desktop:typecheck` | Passed | `tsc --noEmit` 退出码为 0。 |
| IanStage regression | `npm run desktop:test -- IanStage.test.tsx` | Passed | 1 file / 13 tests passed。 |

## Acceptance Criteria Results

- [x] Rust Core 在收到有效 `screen.bounds` 后保存屏幕边界到当前状态。
- [x] 空闲超过阈值且不在 quiet、拖拽、输入、run、night 等保护场景时，`time.tick` 可输出 `appearance.scale_to` 和屏幕边缘 `movement.move_to`。
- [x] 巡游路径从左上附近开始，并沿屏幕边缘产生后续目标。
- [x] 用户互动事件会恢复视觉 scale 到 `1.0`。
- [x] React reducer 能执行 `appearance.scale_to`，且不会覆盖用户持久 `surface_scale` 设置。
- [x] Rust 行为测试、前端 reducer 测试、类型检查通过。

## Failures or Gaps

未执行 Tauri dev 实机 smoke；本次验证覆盖协议、Rust 行为、前端状态和类型检查，但未观察真实桌面巡游视觉效果。

## Follow-Ups

- 用 `npm run desktop:tauri -- dev` 观察 60 秒空闲后的真实桌面巡游；如路径边距或缩小比例不理想，再调 `PATROL_MARGIN`、`WINDOW_SAFE_WIDTH` 或 `scale`。
