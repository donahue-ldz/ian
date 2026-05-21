# Spec: Developer Snooze And Busy Policy

## 状态

已实现，待验收。

## 问题 / 目标

Developer Rhythm 反应必须随时可暂停。0048 目标是增加 snooze 和 busy policy：用户可以暂时关闭开发者反应；系统在忙碌类别或安静时段自动降级。

## 当前产品阶段

v0.2 Developer Rhythm。

## 产品范围

- 设置面提供 Developer Rhythm snooze。
- 支持固定时长暂停，例如 30 分钟、2 小时、今天。
- quiet hours、meeting、presentation、focus 自动降低开发者反应。
- snooze 只影响开发者反应，不影响基础点击互动。

## 明确不做什么

- 不做复杂日程规则。
- 不读取日历。
- 不读取系统专注模式。
- 不永久关闭核心生命感。

## 用户体验

用户忙时可以让 Ian 暂时别对开发事件反应，但仍能点击、拖拽和陪伴。

## 架构约束

- Snooze 状态存本地配置或 runtime state。
- Rust Core policy 执行降级。
- React 只修改 snooze 设置。

## 数据 / 协议变化

新增 developer rhythm snooze 配置：enabled / until_ms / reason。默认关闭。

## 隐私与安全边界

只使用用户显式设置、本地时间和低敏 category。

## 验收标准

- [ ] 用户可开启和关闭 Developer Rhythm snooze。
- [ ] snooze 期间 Git / build / keyboard / app category 反应不产生用户可见气泡。
- [ ] 基础点击和生命行为仍可用。
- [ ] quiet hours 和 busy category 会自动降级开发者反应。
- [ ] 测试覆盖 snooze 到期、手动关闭、基础互动不受影响。

## 验证方式

- Rust config / policy tests。
- Frontend settings tests。
- Browser smoke：snooze UI 和 mock event 降级。
