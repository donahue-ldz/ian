# Decisions: Pet Size Controller

## Decision Log

| Date | Decision | Reason | Impact |
| --- | --- | --- | --- |
| 2026-05-21 | 大小控制器采用 `- / range / + / 百分比` 组合。 | 用户选择该方案；它同时支持点按和拖动，比单独窄滑条更可操作。 | 高级设置里会多一行紧凑复合控件，但不改变底层设置字段。 |
| 2026-05-21 | 继续使用 `0.8` 到 `1.4`、步进 `0.1`。 | 该范围已由现有 UI、Rust clamp 和配置持久化支持。 | 不需要协议、存储或 Rust 迁移。 |
| 2026-05-21 | 自动测试用纯函数覆盖步进和 clamp，用静态渲染覆盖控件结构。 | 当前 Vitest 环境是 Node，没有 DOM 测试运行时；为一个小设置控件新增测试依赖收益不足。 | Browser smoke 负责验证真实点击路径。 |

## Scope Changes

None。

## Deferred Work

- 不做资源包独立尺寸策略。
- 不做窗口大小随 Ian 尺寸自动改变。
- 不做多宠物尺寸配置。
