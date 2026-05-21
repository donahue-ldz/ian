# 实现计划: Puppy Resource Pack + Micro Life Upgrade

对应 spec：`docs/sdd/specs/0082-puppy-micro-life-upgrade/spec.md`

## 状态

已执行；第一轮视觉和速度验收反馈已纳入。

## 总体方案

把本任务拆成三条可独立验证的实现线：

1. 新增 `ian-puppy` 资源包，先让视觉形象和现有动画语义兼容。
2. 在 Rust Core 增加短期内存态和微动作 / 短反应链策略。
3. 在 React 侧只补充 action reducer / animation fallback / smoke 测试，确保前端仍只执行 `IanAction`。

不引入新外部依赖，不实现长期 Mood / Bond，不实现多宠物同时出现。

## 预计改动文件

### 新增

- `apps/desktop/public/resources/pets/ian-puppy/pet.json`
- `apps/desktop/public/resources/pets/ian-puppy/animations.json`
- `apps/desktop/public/resources/pets/ian-puppy/expressions.json`
- `apps/desktop/public/resources/pets/ian-puppy/sprite.svg`
- 可选：`apps/desktop/src-tauri/src/domain/behavior/momentary_life_state.rs`

### 修改

- `apps/desktop/src/resources/resourceLoader.test.ts`
- `apps/desktop/src/renderer/AnimationPlayer.test.ts`
- `apps/desktop/src/state/ianActions.ts`
- `apps/desktop/src/state/ianActions.test.ts`
- `apps/desktop/src/lib/tauriBridge.ts`
- `apps/desktop/src-tauri/src/domain/behavior/behavior_engine.rs`
- `apps/desktop/src-tauri/src/domain/behavior/behavior_policy.rs`
- `apps/desktop/src-tauri/src/domain/behavior/mod.rs`
- `apps/desktop/src-tauri/src/core/creature_state.rs`
- `apps/desktop/src-tauri/src/protocol/state.rs`
- `apps/desktop/src-tauri/src/protocol/action.rs`，仅在确需新增语义动画时修改
- `apps/desktop/src/protocol/generated.ts`，由 Rust 类型生成或同步更新
- `apps/desktop/src-tauri/src/storage/config.rs`
- `docs/resources/resource-pack-authoring.md`，如新增建议动画或 fallback 说明
- `docs/sdd/specs/0082-puppy-micro-life-upgrade/decisions.md`
- `docs/sdd/specs/0082-puppy-micro-life-upgrade/verification.md`

## 实现步骤

### 1. 资源包测试先行

1. 在 `resourceLoader.test.ts` 增加测试，要求 `ian-puppy` 的 `pet.json` 和 `animations.json` 可加载并通过校验。
2. 增加断言：
   - `pet.id === "ian-puppy"`
   - `capabilities` 包含 `idle/rest/walk/sleep/run/zoomies/happy`
   - `idle` 至少包含 2 个 frame
   - `run` 和 `zoomies` 不使用完全相同 frame 序列
3. 运行：
   - `npm run desktop:test -- resourceLoader.test.ts`
4. 预期：
   - 新测试先失败，因为资源包尚不存在。

对应验收：

- `ian-puppy` Resource Pack 存在。
- `ian-puppy` 通过 resource loader 校验。
- 主要动画可区分。

### 2. 新增 `ian-puppy` Resource Pack

1. 创建 `apps/desktop/public/resources/pets/ian-puppy/`。
2. 创建 `pet.json`：
   - `id`: `ian-puppy`
   - `name`: `Ian Puppy`
   - `species`: `puppy`
   - `defaultPersonality`: `gentle_playful`
   - `sprite`: `sprite.svg`
   - `animations`: `animations.json`
   - `expressions`: `expressions.json`
3. 创建 `animations.json`，使用现有 96x96 frame contract。
4. 创建 `sprite.svg`：
   - 小柴犬视觉方向。
   - 帧序列包含 idle 眨眼 / 呼吸 / 尾巴摇的变化。
   - walk/run/zoomies/rest/sleep/happy 可区分。
5. 创建 `expressions.json`，至少包含 `idle`、`happy`、`sleepy`。
6. 运行：
   - `npm run desktop:test -- resourceLoader.test.ts AnimationPlayer.test.ts`

对应验收：

- Puppy 资源完整。
- 旧动画 player contract 不破坏。
- 旧资源包缺少新视觉能力仍 fallback。

### 3. 默认资源包策略

1. 检查 `storage/config.rs`、`tauriBridge.ts` 和前端初始状态中的默认 pack。
2. 将“无用户配置时”的默认 resource pack 调整为 `ian-puppy`。
3. 保持已有用户配置优先，不写迁移强制覆盖。
4. 增加或调整测试，覆盖：
   - 默认 state 使用 `ian-puppy`。
   - 已存在 config 中的 `active_resource_pack` 不被覆盖。
5. 运行：
   - `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml storage`
   - `npm run desktop:test -- ianActions.test.ts`

对应验收：

- 新安装默认显示 `ian-puppy`。
- 已有用户配置不被强制覆盖。

### 4. 设计短期日常性格状态

1. 新增或内联定义 Rust Core 内部状态，例如 `MomentaryLifeState`。
2. 状态字段保持低敏和短期：
   - `last_affection_ms`
   - `last_wake_ms`
   - `last_run_end_ms`
   - `last_drag_end_ms`
   - `ignored_since_ms`
3. 提供方法：
   - `record_event(event, now_ms)`
   - `record_actions(actions, now_ms)`
   - `is_recently_affectionate(now_ms)`
   - `is_recently_awake(now_ms)`
   - `is_recently_run_settling(now_ms)`
   - `expire(now_ms)`
4. 不把该状态写入 SQLite。
5. 增加 Rust 单元测试：
   - 状态在 5-10 分钟窗口内有效。
   - 超时后过期。
   - 重建 `BehaviorEngine` 后状态为空。

对应验收：

- Rust Core 维护短期日常性格状态。
- 状态不持久化。
- 状态可影响后续反馈。

### 5. 微动作策略

1. 在 `BehaviorPolicy` 增加微动作候选和间隔规则。
2. 在 `BehaviorEngine::actions_for_tick` 中，在 scheduler 普通 idle 之后、roam 之前加入低频微动作候选。
3. 微动作只输出轻量 action：
   - `animation.play happy/rest/idle`
   - 可选低强度 `effect.play`
   - 默认不强制打开 bubble。
4. 阻断条件：
   - quiet mode
   - quiet hours
   - bubble input active
   - dragging
   - running / zooming / cooling down
   - focus / meeting / presentation app category
5. 增加 Rust 测试：
   - normal/lively idle tick 可触发微动作。
   - quiet/input/dragging 阻断。
   - 微动作不会输出大幅 movement。

对应验收：

- idle tick 可触发低打扰微动作。
- 安静和交互中不会触发。

### 6. 短反应链升级

1. 扩展 `actions_for_touch_reaction`：
   - DragEnd：`movement.move_to` -> `effect.play` -> `animation.play happy/rest` -> `speech.show`
   - RunAround / Zoomies end：使用 `playful.state Settling` 和短 settle action。
   - MouseNear：先轻动画 / 特效，再可选短句，并保留冷却。
2. 如果新增 `settle` 动画语义，必须更新 Rust protocol、TS generated、resource fallback 和旧资源包测试。
3. 优先复用 `happy/rest/idle` 避免过早扩大协议。
4. 增加 Rust 测试：
   - 拖拽放下输出 2-4 步短反应链。
   - 跑动或 zoomies 后进入 settling，再回 idle。
   - 用户输入或拖拽中不会覆盖交互。

对应验收：

- 拖拽放下短链路。
- 跑动 / zoomies settle。
- React 不接管行为决策。

### 7. 短期状态影响反馈

1. 让 `MomentaryLifeState` 参与至少两类选择：
   - 刚睡醒：点击或靠近时使用更轻短句，例如“刚醒。”。
   - 刚跑完：下一次 tick 或 click 先 settle / 喘一下，而不是立即高能。
   - 刚被摸：连续点击前几次更亲近，过期后恢复普通。
2. 增加 Rust 测试：
   - `recently_awake` 改变点击或靠近短句。
   - `recently_run_settling` 改变下一次 tick 的动作。
   - 过期后恢复普通行为。

对应验收：

- 短期状态影响至少两类反馈。
- 不引入长期 Mood / Bond。

### 8. 前端执行与 fallback 验证

1. 检查 `reduceIanActions` 对 `animation.play`、`effect.play`、`playful.state`、`movement.move_to` 的处理是否足够。
2. 如无需新增 action，不改 reducer 逻辑，只补测试覆盖短链路 action sequence。
3. 如新增 animation name，补 `AnimationPlayer` fallback 测试。
4. 运行：
   - `npm run desktop:test -- ianActions.test.ts AnimationPlayer.test.ts`

对应验收：

- React 只执行 Rust Core 输出。
- 旧资源包 fallback 稳定。

### 9. 文档与验证记录

1. 更新 `docs/resources/resource-pack-authoring.md`：
   - 说明 `ian-puppy` 是 bundled example。
   - 说明微动作建议动画可以 fallback 到现有语义。
2. 更新 `decisions.md`：
   - 是否新增 animation name。
   - 为什么短期状态不持久化。
   - 默认 pack 如何处理已有配置。
3. 更新 `verification.md`：
   - 记录所有测试命令和结果。
   - 记录 browser / desktop smoke 结果。

对应验收：

- 文档和验证完整。
- 剩余风险明确。

## 兼容性说明

- 旧资源包不需要立刻补齐新视觉帧。
- 任何新增动画语义都必须 fallback 到 `idle`、`happy` 或 `run`。
- 已有用户配置优先级高于新默认值。
- 短期状态不做 migration。

## 风险与回滚

- 视觉风险：SVG sprite 可能不够最终美术。回滚方式是保留 `ian-puppy` 但不设为默认。
- 行为风险：微动作太频繁会打扰。回滚方式是提高间隔、只在 lively 开启或关闭自发微动作。
- 协议风险：新增 animation name 会扩大类型面。优先复用现有动画名。
- 配置风险：默认 pack 调整不能覆盖用户选择。实现时必须用测试锁住。

## 验证命令

```bash
npm run desktop:test -- resourceLoader.test.ts AnimationPlayer.test.ts ianActions.test.ts
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml protocol
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml storage
```

Browser / desktop smoke：

- 默认打开显示 puppy。
- idle 2-3 分钟内出现低打扰微动作。
- 双击跑动后有 settle 过渡。
- 拖拽放下后有短反应链。
- quiet mode / 输入中不触发自发微动作。
