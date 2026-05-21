# Spec: Desktop Movement Execution

## 状态

已实现，待验收。

## 问题 / 目标

当前 Ian 已有 `movement.move_to` 协议，但前端基本没有把它执行成真实位移。0021 目标是打通 Rust Core `IanAction::MovementMoveTo` 到桌面窗口或渲染层位移的执行链路，让后续跑动、游走、回窝都复用同一条动作管线。

## 当前产品阶段

v0.1.x Core Life。

## 产品范围

- React action 层能识别并记录 movement action。
- Tauri 环境下通过窗口定位执行短距离位移。
- 浏览器 fallback 下用渲染层 transform 模拟位移，方便测试。
- 位移执行不改变行为决策归属：React 只执行 Rust Core 输出。

## 明确不做什么

- 不实现自主游走策略。
- 不实现复杂路径规划。
- 不读取屏幕内容、窗口标题或其他 App 信息。
- 不做跨屏、多宠物或物理碰撞系统。

## 用户体验

当 Rust Core 输出移动动作时，Ian 能在桌面上平滑移动到目标位置，而不是只切换动画。

## 架构约束

- Rust Core 仍是行为大脑。
- `movement.move_to` 是唯一入口，不允许 React 自己决定去哪里。
- Tauri window API 只作为动作执行器使用。
- 保存位置的逻辑继续走已有配置边界。

## 数据 / 协议变化

优先复用现有 `IanAction::MovementMoveTo { x, y, speed }`。如需补充执行状态，只能新增内部 view state，不新增用户可见复杂状态。

## 隐私与安全边界

只移动 Ian 自身窗口或自身渲染节点，不读取外部窗口、屏幕文字或系统活动信息。

## 验收标准

- [ ] `movement.move_to` 在 React action 层有明确执行路径。
- [ ] Tauri 环境下可移动 Ian 窗口到目标坐标。
- [ ] 浏览器 fallback 下可用 transform 验证位移结果。
- [ ] React 不生成新的移动目标，只消费 Rust Core action。
- [ ] 前端测试覆盖 movement action reducer 或 executor。

## 验证方式

- `npm run desktop:test`
- `npm run desktop:typecheck`
- `npm run desktop:build`
- Tauri 本地 movement smoke。
- 浏览器 smoke：确认 transform 或位置状态变化且 console 无 error。
