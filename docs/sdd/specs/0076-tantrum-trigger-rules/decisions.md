# 0076 决策记录

## 2026-05-21

- 首版撒泼只在活泼模式且玩闹能量为高时触发，避免默认用户被打扰。
- 撒泼动作使用短中文气泡、脸红特效、快速左右移动和回到 `idle` 表达，不新增 `tantrum` 协议或动画枚举。
- 撒泼诊断使用 `idle_tantrum`、`tantrum_roll` 这类低敏 key，便于调试且不记录外部内容。
- 冷却复用 `PlayfulState::CoolingDown`，避免与 zoomies 连续叠加。
