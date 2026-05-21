# 0086 实施计划

## 实现步骤

1. 写 Rust 失败测试：追鼠标成功时必须返回多段 `movement.move_to`，并保留 run / speed_lines / diagnostic / cooldown。
   - 对应验收标准：多段追逐、动作链完整。
2. 写 Rust 失败测试：追鼠标冷却约 10 秒，冷却内返回 `blocked_cooldown`，冷却后可再次触发。
   - 对应验收标准：冷却缩短但仍受控。
3. 写 Rust 失败测试：普通模式 idle 微动作在更短窗口出现，安静模式仍阻断。
   - 对应验收标准：idle 存在感增强、安静模式不打扰。
4. 修改 `BehaviorEngine::actions_for_pointer_chase`：
   - 计算 base target、overshoot target、settle target。
   - 返回 2-3 段 `MovementMoveTo`。
   - 将 `PlayfulState::CoolingDown` 结束时间调整为 `now_ms + 10_000`。
5. 修改 `BehaviorPolicy::should_emit_micro_motion`：
   - 普通 / 活泼模式更容易出现微动作。
   - 安静模式保持 false。
6. 运行针对性测试、全量验证和真实桌面端验收。
7. 更新 `verification.md`。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/behavior_engine.rs`
- `apps/desktop/src-tauri/src/domain/behavior/behavior_policy.rs`
- `apps/desktop/src-tauri/src/storage/config.rs`
- `apps/desktop/src/state/ianActions.test.ts`
- `docs/sdd/specs/0086-desktop-liveliness-tuning/spec.md`
- `docs/sdd/specs/0086-desktop-liveliness-tuning/plan.md`
- `docs/sdd/specs/0086-desktop-liveliness-tuning/decisions.md`
- `docs/sdd/specs/0086-desktop-liveliness-tuning/verification.md`

## 接口和模块影响

- 不新增协议类型。
- 不新增持久化字段。
- Rust Core 行为输出仍使用已有 `IanAction`。
- React 执行动作序列的机制保持不变。
- `ConfigFile -> IanState` 只补齐并行变更新增字段的默认值，保证桌面壳可编译启动。

## 兼容性说明

- 现有 `mouse.chase_candidate` 事件继续兼容。
- 浏览器 fallback 不作为桌面验收依据。
- 安静模式、拖动、输入中阻断追逐的行为必须保持。

## 验证命令和桌面验收

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml pointer_chase`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml micro_motion`
- `npm run desktop:test -- ianActions.test.ts`
- `npm run desktop:typecheck`
- `npm run desktop:test`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- 启动或重启 Tauri 桌面壳：`npm run desktop:tauri -- dev --host 127.0.0.1`
- 桌面验收动作：点击 Ian，移开鼠标，观察是否出现多段追逐；若不出现，记录诊断或运行日志中的阻断原因。

## 风险和回滚

- 风险：冷却过短导致打扰。当前仅用于体验验证，后续可调回更长冷却。
- 风险：多段移动过夸张。用 movement boundary 约束每段目标。
- 回滚：恢复单段追逐和 75 秒冷却，恢复微动作原节奏。
