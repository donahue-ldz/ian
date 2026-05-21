# Spec: Reminder Engine

## 状态

已实现，待验收。

## 问题 / 目标

Ian 可以进入温和提醒阶段，但提醒必须服务陪伴感，而不是生产力工具。0012 目标是建立本地、低频、可关闭的 Reminder Engine，先支持喝水/休息类轻提醒。

## 当前产品阶段

v0.1.x Product Iteration。

## 产品范围

- Rust Core 定义 ReminderPolicy 和 ReminderEngine。
- 基于本地时间和用户设置产生低频提醒。
- 提醒输出为短气泡和轻动画。
- 用户可在设置中关闭提醒。

## 明确不做什么

- 不读取键盘节奏。
- 不读取日历、Feishu、IDE、Git。
- 不做任务管理。
- 不做强提醒、弹窗轰炸或系统通知。

## 用户体验

Ian 偶尔用短句提醒用户喝水或休息，并且用户可以关闭。

## 架构约束

- ReminderEngine 属于 Rust Core。
- Reminder 由 `time.tick` 和本地配置驱动。
- React 只渲染 `speech.show` / animation action。

## 数据 / 协议变化

可新增 reminder config 和 reminder repository；可复用 `speech.show` 输出。

## 隐私与安全边界

提醒只基于本地时间，不读取工作内容或外部应用。

## 验收标准

- [ ] Rust Core 存在 ReminderEngine 和 ReminderPolicy。
- [ ] Reminder 可通过设置关闭。
- [ ] `time.tick` 在满足间隔时可产生喝水/休息短气泡。
- [ ] 提醒有冷却时间，不能连续刷屏。
- [ ] 不读取键盘、窗口、Git 或外部应用信息。

## 验证方式

- Rust reminder 单元测试。
- 设置持久化测试。
- Playwright smoke 验证提醒显示和关闭后不显示。
