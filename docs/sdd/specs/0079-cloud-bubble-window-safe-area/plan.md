# 0079 Cloud Bubble Window Safe Area Plan

## 实现步骤

1. 增加回归测试，覆盖桌面窗口上下文标记、桌面底部贴近布局、浏览器预览居中布局和 Tauri 窗口高度。
   - 对应验收标准：1、2、3。
2. 更新 `IanStage`，为根节点输出桌面 / 预览上下文。
   - 对应验收标准：2、3。
3. 更新 `ianStage.css`，仅在桌面窗口上下文中让舞台内容靠近窗口底部。
   - 对应验收标准：2、3、4。
4. 更新 `tauri.conf.json`，增加透明窗口高度，为头顶气泡预留安全空间。
   - 对应验收标准：1、2。
5. 运行验证并记录结果。
   - 对应验收标准：1、2、3、4。

## 预计改动文件

- `apps/desktop/src/renderer/IanStage.tsx`
- `apps/desktop/src/renderer/ianStage.css`
- `apps/desktop/src/renderer/IanStage.test.tsx`
- `apps/desktop/src/renderer/Bubble.view.test.tsx`
- `apps/desktop/src-tauri/tauri.conf.json`
- `docs/sdd/specs/0079-cloud-bubble-window-safe-area/verification.md`

## 接口 / 模块影响

- React DOM 增加 `data-window-context` 标记。
- Tauri 主窗口高度调整；窗口仍透明、无装饰、不可缩放、always-on-top。
- Rust 协议和行为模块不变。

## 兼容性说明

窗口高度增加会让透明点击区域变高，但 Ian 视觉主体仍贴近窗口底部。已有位置保存逻辑按窗口位置工作，不需要数据迁移。

## 风险和回滚

- 风险：窗口变高后与屏幕边缘的透明区域占用更多空间。
- 回滚：恢复 Tauri 窗口高度与桌面上下文 CSS。

## 执行结果

- 已在 `IanStage` 根节点增加 `data-window-context`，区分桌面窗口和浏览器预览。
- 已将桌面窗口高度从 `260` 调整为 `340`，为气泡顶部云朵瓣预留空间。
- 已让桌面上下文舞台底部对齐，避免 Ian 在增高透明窗口中悬空。
- 已保留浏览器预览居中布局。
