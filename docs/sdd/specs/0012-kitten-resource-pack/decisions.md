# Decisions: Kitten Resource Pack

| Date | Decision | Rationale | Scope Impact |
| --- | --- | --- | --- |
| 2026-05-21 | 新建 `ian-kitten` Resource Pack，而不是直接覆盖 `ian-alpaca`。 | 用户明确选择“新建”；资源包 id 应表达宠物身份，避免羊驼包中实际放小猫。 | 需要新增资源包目录，并把默认 pet/resource pack 切到 `ian-kitten`。 |
| 2026-05-21 | `ian-alpaca` 保留为备用资源包，不在本任务删除。 | 删除旧资源包会扩大范围并增加回滚风险；P0 只需要默认显示小猫。 | 不做资源清理，不做多宠物 UI。 |
| 2026-05-21 | 小猫资源先使用 SVG sprite sheet。 | 当前渲染器已经支持 SVG sprite sheet，改动小、可版本化、可测试；正式 PNG 美术可后续替换。 | 不引入图片生成运行时或额外依赖。 |
| 2026-05-21 | 桌面拖动改用屏幕坐标驱动原生 Tauri 窗口移动。 | 对照 Tauri 官方拖动说明和 OpenPets 实现后，确认 `clientX/clientY` 会随窗口移动重置，导致拖动像被限制在小窗口内；成熟实现使用屏幕坐标 delta 移动原生窗口。 | React 仍只负责采集输入和执行窗口动作；浏览器预览保留原内部拖动效果。 |
| 2026-05-21 | 启用 Tauri `core:window:allow-start-dragging`，并在桌面拖动开始时请求原生窗口拖动。 | Tauri 官方推荐 `startDragging()` / `data-tauri-drag-region` 走原生窗口拖动；手动屏幕坐标移动保留为兜底。 | 新增最小窗口权限，不开放额外系统能力。 |
