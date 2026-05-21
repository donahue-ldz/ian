# Verification: Idle Tiny Perimeter Patrol

## Spec

`docs/sdd/specs/0087-idle-tiny-perimeter-patrol/spec.md`

## Verification Summary

已完成 RED/GREEN 验证。Rust 测试先因缺少 `ScreenBounds` / `AppearanceScaleTo` 失败，前端 reducer 测试先因缺少 `appearanceScale` 失败；实现协议、行为和前端执行后，完整 Rust 测试、相关前端测试和类型检查通过。实机反馈后补充验证发现桌面体验缺陷：巡游只发一个角点、远距离移动仍固定时长、巡游期间其他自主动作会插队导致来回位移和闪烁；新增失败测试覆盖后已修复。

## Checks

| Check | Command / Method | Result | Notes |
| --- | --- | --- | --- |
| Rust RED behavior | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior` | Failed as expected | 编译失败，缺少 `ScreenBounds`、`screen_bounds` state 字段和 `AppearanceScaleTo` action。 |
| Frontend RED reducer | `npm run desktop:test -- ianActions.test.ts` | Failed as expected | 1 failed / 13 passed；`appearanceScale` 尚未实现。 |
| Rust behavior | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior` | Passed | 51 behavior-related tests passed。 |
| Full Rust tests | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` | Passed | 110 tests passed。 |
| RED full patrol loop | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml idle_perimeter_patrol` | Failed as expected | 失败显示只输出 `[(32.0, 32.0)]`，没有完整边缘路径。 |
| RED distance-based movement duration | `npm run desktop:test -- ianActions.test.ts` | Failed as expected | 失败原因是 `durationForDesktopMovement is not a function`。 |
| Rust behavior after patrol fix | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior` | Passed | 52 behavior-related tests passed。 |
| Frontend reducer | `npm run desktop:test -- ianActions.test.ts` | Passed | 1 file / 15 tests passed。 |
| Frontend typecheck | `npm run desktop:typecheck` | Passed | `tsc --noEmit` 退出码为 0。 |
| IanStage regression | `npm run desktop:test -- IanStage.test.tsx` | Passed | 1 file / 13 tests passed。 |
| RED desktop visual offset | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml idle_perimeter_patrol` | Failed as expected | 失败显示首个窗口目标未补偿透明窗口内部视觉偏移。 |
| RED active patrol tick interruption | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml perimeter_patrol` | Failed as expected | `Settling` 期间 90s tick 仍发 `AnimationPlay { name: "sleep" }`。 |
| RED active patrol pointer interruption | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml perimeter_patrol` | Failed as expected | `MouseChaseCandidate` 在巡游期间仍发 `run` 和多段 `MovementMoveTo`。 |
| RED frontend movement batch guard | `../../node_modules/.bin/vitest run ianActions.test.ts` from `apps/desktop` | Failed as expected | 失败原因是 `createMovementSequenceGuard is not a function`。 |
| Patrol regression | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml idle_perimeter_patrol` | Passed | 4 patrol tests passed。 |
| Rust behavior after interruption fix | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior` | Passed | 56 behavior-related tests passed。 |
| Full Rust tests | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` | Passed | 115 tests passed。 |
| Full frontend tests | `../../node_modules/.bin/vitest run` from `apps/desktop` | Passed | 12 files / 76 tests passed。 |
| Frontend typecheck after interruption fix | `../../node_modules/.bin/tsc --noEmit` from `apps/desktop` | Passed | 退出码为 0。 |
| Tauri desktop smoke attempt | `../../node_modules/.bin/tauri dev`; then existing desktop process + `screencapture -x /tmp/ian-desktop-smoke.png` | Inconclusive | 新启动失败，因为 `1420` 已被现有 `vite` / `ian_desktop` 占用；等待 70s 后全屏截图未清晰显示 Ian 桌面窗口，不能记为通过。 |

## Acceptance Criteria Results

- [x] Rust Core 在收到有效 `screen.bounds` 后保存屏幕边界到当前状态。
- [x] 空闲超过阈值且不在 quiet、拖拽、输入、run、night 等保护场景时，`time.tick` 可输出 `appearance.scale_to` 和屏幕边缘 `movement.move_to`。
- [x] 巡游路径从左上附近开始，并沿屏幕边缘产生后续目标。
- [x] 巡游目标补偿桌面透明窗口内部偏移，使缩小后的可见小动物贴近屏幕边缘。
- [x] 巡游 active 期间，Rust Core 不再输出会打断巡游的自主动画、micro motion、roam 或 pointer chase movement。
- [x] React 桌面移动执行层会取消旧 movement 批次，避免多个窗口移动循环互相拉扯。
- [x] 用户互动事件会恢复视觉 scale 到 `1.0`。
- [x] React reducer 能执行 `appearance.scale_to`，且不会覆盖用户持久 `surface_scale` 设置。
- [x] Rust 行为测试、前端 reducer 测试、类型检查通过。

## Failures or Gaps

Tauri dev 实机 smoke 已尝试但结果不具备结论：当前环境已有桌面进程占用 `1420`，等待空闲周期后的全屏截图未清晰显示 Ian 桌面窗口。本次可靠验证覆盖协议、Rust 行为、前端状态、桌面移动批次守卫和类型检查；真实桌面巡游观感仍需在用户当前桌面窗口里复核。

## Follow-Ups

- 用 `npm run desktop:tauri -- dev` 观察 60 秒空闲后的真实桌面巡游；如路径边距或缩小比例不理想，再调 `PATROL_MARGIN`、`WINDOW_SAFE_WIDTH` 或 `scale`。
