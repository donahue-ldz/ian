# 0073 实施计划

## 步骤

1. 在 Rust 行为引擎中调整 `IanEvent::AppStarted` 的动作序列。
2. 使用已有 `animation.play`、`effect.play`、`bubble.open`、`speech.show` 表达醒来反馈。
3. 添加单元测试，确保启动动作包含高兴动画、闪光特效和中文短句。
4. 运行 Rust、前端和构建验证。

## 约束

- 不新增协议字段。
- 不移动 Ian 的位置。
- 不增加用户必须处理的交互。

