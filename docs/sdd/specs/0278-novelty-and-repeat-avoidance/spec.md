# 0278 · Novelty And Repeat Avoidance

## 问题 / 目标

惊喜感会因为重复而快速消失。本 SDD 增加轻量 Novelty 策略，避免 Ian 连续重复同一个 Moment、同一句气泡或同一种收尾。

## 当前产品阶段

v0.1.x Life Feel / Novelty Policy。

## 产品范围

- 记录短期最近播放过的 Moment kind、story variant、bubble phrase。
- Moment 选择时降低最近重复项权重。
- 对诊断模式保留指定触发能力。
- 不做长期用户画像或复杂推荐。

## 明确不做

- 不做个性化推荐系统。
- 不持久化高敏行为日志。
- 不基于用户工作内容判断新鲜感。
- 不保证每次都不同，避免复杂和不可控。

## 用户体验

Ian 不会连续说同一句话、连续做同一个惊喜。用户更容易感到“今天它有点不一样”。

## 架构约束

- Novelty 策略属于 Rust Core / Behavior Policy。
- React 不根据本地 UI 状态自行随机选择新动作。
- Novelty 不能绕过冷却、用户控制和隐私边界。

## 数据 / 协议变化

优先使用内存短期记录。若后续需要持久化，必须独立 SDD。

## 隐私与安全边界

只记录低敏枚举 ID，不记录原始文本、鼠标轨迹或外部内容。

## 验收标准

- [ ] Core 记录最近 N 个 Moment kind / variant / phrase ID。
- [ ] 连续重复项权重降低或被短期排除。
- [ ] 诊断模式仍可指定触发某个 Moment。
- [ ] 记录内容只包含低敏枚举 ID。
- [ ] 单元测试覆盖重复避免和可用候选不足时的降级。

## 验证方式

- Rust Novelty Policy 单元测试。
- 气泡 phrase 选择测试。
- 真实 Tauri 桌面连续触发观察不重复。
