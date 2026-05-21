# 决策记录: Resource Pack Renderer

## 决策日志

| 日期 | 决策 | 原因 | 影响 |
| --- | --- | --- | --- |
| 2026-05-21 | 0002 聚焦 Resource Pack sprite-sheet renderer。 | `docs/sdd` 示例和 v0.1 milestones 都指向 Resource Pack Renderer 作为 0002。 | 不扩展多宠物、资源包导入或美术系统。 |
| 2026-05-21 | 默认资源继续使用 checked-in SVG sprite sheet。 | 当前没有正式 PNG 美术；SVG 能作为可版本化、可检查的 placeholder sprite sheet。 | 后续可用同一 manifest 替换为 PNG。 |

## 范围变化

暂无。

## 延后工作

- 正式 PNG sprite sheet
- 多宠物选择 UI
- 资源包导入和校验工具
- 声音资源播放
