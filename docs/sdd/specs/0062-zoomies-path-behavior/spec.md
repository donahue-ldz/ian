# Spec: Zoomies Path Behavior

## 状态

已实现，待用户验收。

## 问题 / 目标

已有 run-around 是短路径跑一圈，不满足“满屏乱跑”的强表现。0062 目标是新增 zoomies 路径行为：Ian 在安全边界内快速跑过多个屏幕区域，然后自然停下或回到 anchor。

## 产品范围

- 新增 zoomies action sequence，包含 4-8 个 waypoint。
- waypoint 覆盖更大桌面范围，但必须经过 movement boundary policy。
- zoomies 有明确开始、运行、结束和取消状态。
- zoomies 不替代双击短 run-around，可作为更高能彩蛋。

## 明确不做什么

- 不读取其他窗口位置或屏幕内容。
- 不做物理碰撞和障碍物躲避。
- 不跨越安全边界或跑出屏幕。
- 不在用户拖拽、输入、安静时段中启动。

## 用户体验

用户触发或偶遇彩蛋时，Ian 会突然兴奋地跑过桌面几个位置，像高兴到停不下来，但几秒内会收住。

## 架构约束

- zoomies 路径由 Rust Core 生成。
- React 顺序执行 `MovementMoveTo` 和动画 action。
- 路径必须复用 0024 的边界策略，并与 0029 home anchor 兼容。

## 数据 / 协议变化

可新增 `BehaviorZoomies { duration_ms }`，也可复用 `BehaviorRunAround` 并增加 reason。若新增协议，必须生成 TypeScript 类型。

## 隐私与安全边界

只使用 Ian 当前坐标、home anchor、屏幕安全范围和本地设置。

## 验收标准

- [x] zoomies 包含 4-8 个真实 movement waypoint。
- [x] waypoint 覆盖范围明显大于 0022 run-around。
- [x] 所有 waypoint 都经过边界策略约束。
- [x] zoomies 结束后回 idle 或回 anchor。
- [x] 用户 active interaction 和 quiet mode 阻止启动。

## 验证方式

- Rust behavior tests 覆盖路径数量、边界、结束状态。
- 前端 action sequence 测试。
- Browser / Tauri smoke：触发 zoomies 后位置明显变化并回 idle。
