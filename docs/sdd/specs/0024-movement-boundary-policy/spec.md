# Spec: Movement Boundary Policy

## 状态

已实现，待验收。

## 问题 / 目标

当 Ian 开始真实移动后，必须避免跑出屏幕、挡住用户太久或违背安静模式。0024 目标是建立统一移动边界策略，为 run-around、idle roaming、home return 提供可测试约束。

## 当前产品阶段

v0.1.x Core Life。

## 产品范围

- 定义 movement bounds：屏幕可见区域、边距、最大单次位移。
- BehaviorMode 影响移动幅度和频率。
- 目标点超界时裁剪或拒绝。
- 边界策略可被单元测试覆盖。

## 明确不做什么

- 不读取其他窗口位置。
- 不做避障。
- 不做全局屏幕 OCR。
- 不处理多宠物碰撞。

## 用户体验

Ian 可以动，但不会跑丢、跑出屏幕或持续占据用户无法控制的位置。

## 架构约束

- 边界策略位于 Rust Core policy 或独立 domain 模块。
- Tauri / React 可提供自身窗口尺寸和屏幕尺寸摘要，但不做行为决策。
- 所有 movement action 在输出前应经过边界约束。

## 数据 / 协议变化

可在 `IanState` 或内部 runtime state 中增加 movement bounds 摘要。不得包含外部窗口标题、应用名或屏幕内容。

## 隐私与安全边界

允许读取 Ian 自身窗口尺寸和屏幕工作区尺寸；不读取其他窗口、文件、URL 或屏幕文字。

## 验收标准

- [ ] MovementBoundaryPolicy 或等价模块存在。
- [ ] 超出边界的 movement 目标会被裁剪或拒绝。
- [ ] quiet / normal / lively 对最大位移有不同约束。
- [ ] run-around 和 roam 使用同一边界策略。
- [ ] 测试覆盖边界裁剪、模式差异和负坐标。

## 验证方式

- Rust policy 单元测试。
- 前端 / Tauri smoke：靠近屏幕边缘触发移动不出界。
- `rg` 检查无窗口标题、URL、OCR 采集。
