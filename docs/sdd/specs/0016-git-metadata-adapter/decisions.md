# 决策记录: Git Metadata Adapter

## 决策日志

| 日期 | 决策 | 原因 | 影响 |
| --- | --- | --- | --- |
| 2026-05-21 | Git adapter 只读取低敏元数据且默认关闭。 | Developer Rhythm 需要节奏信号，但不能越过隐私边界。 | diff、代码正文和 commit message 全文不进入事件。 |

## 范围变化

暂无。

## 延后工作

- GitHub/GitLab API
- PR/issue 集成
- 代码内容分析

