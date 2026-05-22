# 0270 · 决策记录

## 2026-05-22

- 生命感二阶段先建立内部动机，不直接加更多用户可见功能。
- Life Drive 暂不持久化，避免和完整 Mood / Bond System 混淆。
- Life Drive 只消费低敏事件种类和坐标类上下文，不保存输入文本、窗口内容、文件路径或外部应用内容。
- Life Drive 的内部数值不进入 `IanState` 或 UI 展示；目前仅通过行为选择影响 `IanAction`。
