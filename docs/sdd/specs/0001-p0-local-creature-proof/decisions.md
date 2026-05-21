# 决策记录: P0 本地数字生命验证版

## 决策日志

| 日期 | 决策 | 原因 | 影响 |
| --- | --- | --- | --- |
| 2026-05-21 | 将 P0 视为产品验证，同时建立 v0.1 Architecture Baseline。 | Ian 需要快速验证产品生命感，但不能形成一次性架构。 | 第一版实现会包含架构 skeleton，但用户可见功能保持克制。 |
| 2026-05-21 | BYOM 保持可选，不作为 P0 核心卖点。 | P0 应验证桌面生命存在感，而不是变成聊天产品。 | Demo Dialogue 必须实现；BYOM 可延后到 v0.1.x，同时先准备 provider boundary。 |
| 2026-05-21 | 未来能力只做 skeleton，不做用户可见功能。 | 防止 P0 吸收 v0.2 / v0.3 范围。 | Git、keyboard rhythm、Feishu、Pet Visit、plugin、Mood/Bond、long-term memory 不进入 P0 行为。 |

## 范围变化

暂无。

## 延后工作

- OpenAI-compatible BYOM UI
- 主动喝水 / 休息提醒
- 完整 Mood System
- 完整 Bond System
- 长期记忆
- Git / build / test 集成
- keyboard rhythm
- active window awareness
- Feishu Relay
- Pet Visit
- plugin system

