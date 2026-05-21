# Decisions: Kitten Resource Pack

| Date | Decision | Rationale | Scope Impact |
| --- | --- | --- | --- |
| 2026-05-21 | 新建 `ian-kitten` Resource Pack，而不是直接覆盖 `ian-alpaca`。 | 用户明确选择“新建”；资源包 id 应表达宠物身份，避免羊驼包中实际放小猫。 | 需要新增资源包目录，并把默认 pet/resource pack 切到 `ian-kitten`。 |
| 2026-05-21 | `ian-alpaca` 保留为备用资源包，不在本任务删除。 | 删除旧资源包会扩大范围并增加回滚风险；P0 只需要默认显示小猫。 | 不做资源清理，不做多宠物 UI。 |
| 2026-05-21 | 小猫资源先使用 SVG sprite sheet。 | 当前渲染器已经支持 SVG sprite sheet，改动小、可版本化、可测试；正式 PNG 美术可后续替换。 | 不引入图片生成运行时或额外依赖。 |
