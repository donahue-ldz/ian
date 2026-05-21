# Implementation Plan: Idle Tiny Perimeter Patrol

## Spec

`docs/sdd/specs/0087-idle-tiny-perimeter-patrol/spec.md`

## Summary

新增低敏 `screen.bounds` event 和临时 `appearance.scale_to` action。Rust Core 保存屏幕 bounds，在空闲 tick 上输出缩小 + 边缘慢速移动；前端发送 bounds 并执行临时 scale。

## Steps

1. 添加 Rust RED 测试：`screen.bounds` 后，空闲 tick 输出 `appearance.scale_to` 和靠近左上角的 slow movement。
2. 添加前端 RED 测试：`appearance.scale_to` 更新 transient scale，`state.sync.surface_scale` 仍作为基础 scale。
3. 更新 Rust protocol 和 checked-in TypeScript protocol。
4. 在 `IanState` 中增加非敏感 `screen_bounds` 和巡游索引状态。
5. 在 runtime 中处理 `screen.bounds` 状态更新和互动恢复 scale。
6. 在 behavior engine tick 中加入空闲巡游 guard 和路径生成。
7. 在 App 启动和周期 tick 时发送当前 monitor bounds。
8. 在 IanStage 中将基础 `surfaceScale` 与临时 scale 相乘。
9. 运行验证并更新 `verification.md`。

## Expected File Changes

- `apps/desktop/src-tauri/src/protocol/event.rs`: 新增 `screen.bounds`。
- `apps/desktop/src-tauri/src/protocol/action.rs`: 新增 `appearance.scale_to`。
- `apps/desktop/src-tauri/src/protocol/state.rs`: 新增 screen bounds / patrol state。
- `apps/desktop/src-tauri/src/core/creature_state.rs`: 状态更新 helper。
- `apps/desktop/src-tauri/src/app/runtime.rs`: 处理 bounds 与互动恢复。
- `apps/desktop/src-tauri/src/domain/behavior/behavior_engine.rs`: 生成 idle perimeter patrol。
- `apps/desktop/src/protocol/generated.ts`: 同步协议类型。
- `apps/desktop/src/lib/position.ts`: 获取当前 monitor bounds。
- `apps/desktop/src/App.tsx`: 周期上报 bounds。
- `apps/desktop/src/state/ianActions.ts`: reducer 执行临时 scale。
- `apps/desktop/src/renderer/IanStage.tsx`: 应用临时 scale。
- `docs/sdd/specs/0087-idle-tiny-perimeter-patrol/*`: SDD 记录。

## Interfaces and Boundaries

React 只负责报告显示器 bounds 和执行 action。Rust Core 根据 `IanState` 与 `IanEvent` 决定是否巡游、下一个目标和缩放动作。`surface_scale` 仍是用户持久设置，临时巡游缩放只存在于 frontend view state。

## Verification Commands

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
npm run desktop:test -- ianActions.test.ts
npm run desktop:typecheck
```

## Risks

- 真实桌面坐标和透明窗口尺寸可能需要进一步调安全边距；本轮先使用保守 32px padding。
- 若巡游过于频繁，可以后续调高空闲阈值或 tick cadence。

## Rollback Notes

可移除 `screen.bounds` 事件、`appearance.scale_to` action 和 behavior tick 中巡游分支，恢复原有本地 roam 行为。
