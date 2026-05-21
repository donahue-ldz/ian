# Verification: Desktop Presence Chrome Reduction

## 状态

已实现，待用户验收。

## 实际验证

- [x] `npm run desktop:test`：通过，7 个 test files / 25 个 tests。
- [x] `npm run desktop:typecheck`：通过，`tsc --noEmit` 退出 0。
- [x] `npm run desktop:build`：通过，Vite production build 完成。
- [x] Browser 默认态截图：`docs/sdd/specs/0051-desktop-presence-chrome-reduction/screenshots/default-dark.jpg`。
- [x] Browser 设置入口截图：`docs/sdd/specs/0051-desktop-presence-chrome-reduction/screenshots/settings-dark.jpg`。
- [x] Browser DOM 检查：默认态存在 `aria-label="打开 Ian 设置"` 的图标按钮；设置面板打开后存在 `aria-label="Ian 设置"`。
- [x] 回归 RED：`npm run desktop:test -- IanStage.test.tsx` 失败，原因是缺少 `.ian-stage[data-window-context="desktop"] .ian-settings` 桌面安全区规则。
- [x] 回归 GREEN：`npm run desktop:test -- IanStage.test.tsx` 通过，10 个 tests。
- [x] 设置相关前端测试：`npm run desktop:test -- IanStage.test.tsx SettingsPanel.view.test.tsx` 通过，2 个 test files / 12 个 tests。
- [x] 前端全量测试：`npm run desktop:test` 通过，12 个 test files / 62 个 tests。
- [x] `npm run desktop:typecheck`：通过，`tsc --noEmit` 退出 0。
- [x] Browser smoke：`http://localhost:1420/` 中点击设置按钮后，DOM 出现 `aria-label="Ian 设置"` 设置面板，`aria-expanded="true"`。

## 结果

默认态不再显示醒目的“设置”文字按钮，设置入口改为带 `aria-label` / `title` 的低干扰图标按钮。浏览器预览中设置面板仍位于 Ian 本体外侧；桌面窗口中设置面板改为窗口内侧展开，避免被 260px 透明窗口边界裁掉。

## 失败或缺口

Codex Browser 可以打开 `http://127.0.0.1:1420/` 并截图，但尝试使用数据页包裹本地页面来模拟浅色桌面背景时被 Browser URL policy 拦截。已完成深色透明背景截图和 CSS 对比检查；浅色真实桌面背景仍建议在 Tauri 窗口中人工复核。

本次修复的 Browser smoke 使用现有 `http://localhost:1420/` dev server；`http://127.0.0.1:5173/` 被 Browser 拦截，另一个临时 `5173` Vite 实例以错误参数启动，未用于验收。
