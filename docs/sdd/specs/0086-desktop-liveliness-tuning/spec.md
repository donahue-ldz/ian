# 0086 桌面灵动感调优

## 目标

让 Ian 在桌面端看起来更像有生命的小宠物，而不是一次性平移的窗口。先聚焦追鼠标和日常微动作节奏，做一个可真实桌面验收的小切片。

## 当前阶段

P0 / MVP + v0.1 Architecture Baseline。该能力属于 P0 生命感增强，不引入复杂 Mood / Bond / 记忆系统。

## 产品范围

- 追鼠标触发后，Ian 执行 2-3 段小步追逐，而不是单段直线移动。
- 追逐包含轻微 overshoot / settle，让动作有“扑一下、停住”的感觉。
- 体验验证阶段将追鼠标冷却缩短到约 10 秒，便于用户连续观察。
- 普通桌面 idle 微动作更容易被看到，但仍不在安静模式、拖动、输入、运行中打扰用户。
- 保留 `playful.diagnostic`，用于判断追逐是否触发或被距离 / 冷却挡住。

## 明确不做

- 不新增全局鼠标监听。
- 不读取窗口内容、屏幕内容、应用内容或鼠标轨迹历史。
- 不实现完整情绪系统、长期记忆或复杂性格模型。
- 不改宠物资源包形象，不重做动画资产。
- 不移除用户控制、安静模式或现有安全门。

## 用户体验

- 用户点击 Ian 并移开鼠标后，如果追逐判定通过，Ian 会连续跑几小步，更像追上来。
- Ian 不会直接跳到鼠标点，而是向鼠标方向靠近、稍微扑一下、再停住。
- 如果没有触发，诊断仍能说明是太近、太远、冷却或上下文阻断。
- 普通模式下 idle 时偶尔有更明显的小动作，桌面存在感更强。

## 架构约束

- 输入仍然只通过 `IanEvent::MouseChaseCandidate` / `IanEvent::TimeTick` 进入 Rust Core。
- 追逐和微动作判定仍由 Rust Core `BehaviorEngine` 决定。
- React 只执行 Rust 返回的 `IanAction`，不新增行为大脑。
- 不新增持久化字段。

## 数据 / 协议变化

无新增协议类型。继续使用：

- `IanEvent::MouseChaseCandidate`
- `IanEvent::TimeTick`
- `IanAction::MovementMoveTo`
- `IanAction::AnimationPlay`
- `IanAction::EffectPlay`
- `IanAction::PlayfulDiagnostic`
- `IanAction::PlayfulStateSet`

## 隐私与安全边界

- 不读取除当前鼠标坐标以外的桌面信息。
- 不记录鼠标轨迹历史。
- 诊断只记录低敏原因 key，如 `pointer_chase`、`triggered`、`blocked_cooldown`。

## 验收标准

- [x] `MouseChaseCandidate` 触发成功时返回至少 2 段 `movement.move_to`，并且每段目标都不直接等于鼠标坐标。
- [x] 追逐动作包含 `run` 动画、`speed_lines` 效果、`pointer_chase` 诊断和冷却状态。
- [x] 追逐冷却缩短到约 10 秒，冷却内仍返回 `blocked_cooldown` 诊断。
- [x] 距离太近、太远、安静模式、拖动、输入中仍不会触发追逐。
- [x] 普通模式下 idle 微动作节奏更容易观察，安静模式仍不会发出微动作。
- [x] 真实 Tauri 桌面端完成验收：点击 Ian、移开鼠标，观察是否出现多段追逐或记录阻断诊断。

## 验证方式

- Rust 行为测试覆盖追逐多段路径、冷却缩短、阻断条件和微动作节奏。
- 前端测试覆盖多段 `movement.move_to` 的执行节奏不回归。
- 运行 TypeScript 类型检查、前端测试、Rust 测试。
- 启动真实 Tauri 桌面端，执行关键交互并在 `verification.md` 记录观察结果；浏览器预览不能替代该验收。
