# Verification: Movement Personality Tuning

## 状态

已实现，待用户验收。

## 实际验证

- [x] TDD RED：`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior::behavior_engine::tests::movement_personality_controls_autonomous_movement_frequency` 失败符合预期，normal 在 45 秒即发生自主位移。
- [x] Rust behavior tests：`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior` 通过，29 个匹配 tests。
- [x] `npm run desktop:test`：通过，8 个 test files / 30 个 tests。
- [x] `npm run desktop:typecheck`：通过，`tsc --noEmit` 退出 0。
- [x] 人工代码检查：`BehaviorEngine::actions_for_tick` 仍在 `is_user_interaction_active`、quiet hours、busy app、run/sleep guard 后才考虑自主位移。

## 结果

新增 Rust `MovementProfile` 默认参数，quiet 不自主位移，normal 每 135 秒左右才位移，lively 每 75 秒左右位移；45 秒 normal tick 保留 walk 微动作但不移动。自主位移仍经过 `MovementBoundaryPolicy` 约束，并继续避开输入、拖拽、run、sleep、quiet hours 和 busy app 场景。

## 失败或缺口

未执行真实 5 分钟 Browser idle smoke。当前已用 Rust tests 覆盖 45s / 75s / 135s 调度点和互斥状态；后续若 Browser 稳定，可补长时间视觉观察记录。
