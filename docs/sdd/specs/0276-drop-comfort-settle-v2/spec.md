# 0276 · Drop Comfort Settle V2

## 问题 / 目标

Ian 被放下后如果立即静止，会缺少重量和自我安顿感。本 SDD 增强 drop settle，让 Ian 放下后轻微找舒服姿势、确认位置并恢复 idle。

## 当前产品阶段

v0.1.x Life Feel / Settle Feel。

## 产品范围

- 放下后播放短 settle story。
- 根据 Life Drive comfort / energy 选择轻微弹性、拍拍、坐下、看一眼等收尾。
- 放下后更新安全感或舒适度。
- 保证最终位置持久化正确。

## 明确不做

- 不自动大范围重新选位置。
- 不做窗口避障或主动避开其他 app。
- 不读取屏幕内容判断舒服位置。
- 不做长时间动画。

## 用户体验

用户放下 Ian 后，它会轻轻安顿一下，像确认“这里可以”，然后回到 idle。

## 架构约束

- Drop event 进入 Rust Core，Core 决定 settle story。
- React 执行动画、气泡和轻微视觉回弹。
- 最终位置由 position persistence 负责，不被视觉回弹破坏。

## 数据 / 协议变化

复用 Micro Story、Motion Profile 和 Life Drive。必要时新增 settle intent。

## 隐私与安全边界

只使用 Ian 位置、屏幕安全区和低敏事件。

## 验收标准

- [ ] 放下后有短 settle feedback，持续时间可控。
- [ ] settle 不改变最终持久化位置或导致跳屏。
- [ ] 连续拖放不会叠加多个 settle story。
- [ ] reduced motion 下 settle 降级为短气泡或低幅动画。
- [ ] 真实桌面拖放后 Ian 能稳定回到 idle。

## 验证方式

- Rust drop story / interaction gate 测试。
- 前端 position persistence 测试。
- 真实 Tauri 桌面拖放验收。
