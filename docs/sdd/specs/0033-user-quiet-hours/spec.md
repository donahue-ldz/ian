# Spec: User Quiet Hours

## 状态

已实现，待验收。

## 问题 / 目标

用户需要能明确控制 Ian 什么时候少动、少说话。0033 目标是增加本地安静时段，让自主移动、主动气泡、提醒和好奇反应在这些时间内降频或关闭。

## 当前产品阶段

v0.1.x User Control。

## 产品范围

- 本地配置 quiet hours：开始时间、结束时间、是否启用。
- Scheduler 和 BehaviorPolicy 读取 quiet hours。
- quiet hours 优先级高于 lively 模式。
- 设置面可查看和修改最小配置。

## 明确不做什么

- 不读取系统专注模式。
- 不读取日历会议。
- 不自动推断用户忙碌。
- 不做复杂规则引擎。

## 用户体验

用户设置夜间或工作时段后，Ian 会更安静：少移动、少气泡、少主动反应，但点击互动仍可用。

## 架构约束

- quiet hours 存储在本地配置。
- Rust Core 使用 quiet hours 做行为约束。
- React 设置面只读写配置，不自己判断行为。

## 数据 / 协议变化

扩展 config schema，增加 quiet hours 字段。旧配置应有默认禁用或合理默认值。

## 隐私与安全边界

只使用用户显式设置，不读取外部环境。

## 验收标准

- [ ] quiet hours 可配置并持久化。
- [ ] quiet hours 生效时自主 roam / sleep / bubble 主动行为降频或关闭。
- [ ] 用户点击互动不被完全禁用。
- [ ] lively 模式不能绕过 quiet hours。
- [ ] 测试覆盖跨午夜时段、禁用状态、优先级和持久化。

## 验证方式

- Rust config tests。
- Behavior policy tests。
- Frontend settings tests。
- Browser smoke：设置安静时段后 UI 状态正确。
