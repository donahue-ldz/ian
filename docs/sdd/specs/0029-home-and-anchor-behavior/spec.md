# Spec: Home And Anchor Behavior

## 状态

已实现，待验收。

## 问题 / 目标

Ian 需要能动，但也要让用户感觉可控。0029 目标是建立 home / anchor 行为：用户拖拽或设置位置后，Ian 记住一个认可的位置，并在跑动或游走后能回到附近。

## 当前产品阶段

v0.1.x Core Life。

## 产品范围

- 将用户拖拽结束的位置视为 anchor 候选。
- 保存 anchor 到本地配置或状态。
- run-around / roam 可选择回到 anchor 附近。
- 提供最小设置或内部命令重置 anchor。

## 明确不做什么

- 不做多 home 点。
- 不做房间、地图或家具系统。
- 不基于其他窗口自动选择 anchor。
- 不做跨设备同步。

## 用户体验

用户把 Ian 拖到喜欢的位置后，Ian 可以偶尔离开活动，但最终会回到附近，保持“住在这里”的感觉。

## 架构约束

- Anchor 属于本地状态 / 配置。
- Rust Core 决定何时回 anchor。
- React 只负责拖拽事件和 movement 执行。

## 数据 / 协议变化

可在 config 中增加 `home_anchor` 或复用现有 position 作为初始 anchor。需要 migration / default 兼容。

## 隐私与安全边界

只保存 Ian 自身位置，不记录其他窗口、屏幕内容或用户活动。

## 验收标准

- [ ] 拖拽结束后 anchor 可保存。
- [ ] 重启后 anchor 可恢复。
- [ ] run-around / roam 后可回到 anchor 附近。
- [ ] anchor 超出边界时会被 0024 策略修正。
- [ ] 测试覆盖保存、恢复、回 anchor 和边界修正。

## 验证方式

- Rust storage config tests。
- Behavior tests。
- Tauri smoke：拖动、重启、回到 anchor 附近。
