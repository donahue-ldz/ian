# 0079 Cloud Bubble Window Safe Area Verification

## 状态

已验证。

## RED

- `npm run desktop:test -- IanStage.test.tsx`
  - 结果：失败，符合预期。
  - 失败点：缺少 `data-window-context="desktop"` / `preview`；缺少桌面安全区 CSS；Tauri 窗口高度仍为 `260`。

## GREEN / 回归

- [x] `npm run desktop:test -- IanStage.test.tsx Bubble.view.test.tsx`
  - 结果：通过，2 个文件 / 12 个测试。
- [x] `npm run desktop:test`
  - 结果：通过，11 个文件 / 52 个测试。
- [x] `npm run desktop:typecheck`
  - 结果：通过。
- [x] `npm run desktop:build`
  - 结果：通过，Vite 产物生成成功。
- [x] 浏览器视觉检查
  - URL：`http://127.0.0.1:1420/?v=0079`
  - 截图：`docs/sdd/specs/0079-cloud-bubble-window-safe-area/screenshots/0079-cloud-bubble-safe-area.png`
  - Smoke 数据：`docs/sdd/specs/0079-cloud-bubble-window-safe-area/screenshots/0079-cloud-bubble-safe-area-smoke.json`
  - 结果：浏览器预览仍为 `data-window-context="preview"`，气泡保持 `borderColor: rgba(17, 17, 17, 0.64)`、`borderWidth: 3px`、`cloudSize: short`。

## 剩余风险

- 真实 macOS 透明 Tauri 窗口的最终裁切效果依赖 Tauri 运行时窗口尺寸；已通过配置与布局测试覆盖，仍建议用户在桌面窗口里验收一次。
