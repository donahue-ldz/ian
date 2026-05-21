# 决策记录: Local State 与基础设置持久化

## 决策日志

| 日期 | 决策 | 原因 | 影响 |
| --- | --- | --- | --- |
| 2026-05-21 | 0005 聚焦最小 local-first 状态，不做长期记忆。 | P0 需要重启恢复，但不能提前进入 Memory System。 | 只持久化 position/config，SQLite 保留 skeleton。 |

## 范围变化

暂无。

## 延后工作

- 长期记忆
- Mood/Bond 历史
- reminder records UI
- 多宠物管理
- 云同步

