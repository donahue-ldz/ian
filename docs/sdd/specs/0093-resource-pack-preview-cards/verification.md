# 0093 · 验证记录

## 2026-05-21

### 自动化验证

- RED：新增 `SettingsPanel.view.test.tsx` 预览卡测试后失败，缺少 `ian-pet-preview-grid` 和切换按钮。
- GREEN：`PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run test --workspace @ian/desktop -- src/resources/resourceLoader.test.ts src/renderer/SettingsPanel.view.test.tsx src/App.test.ts src/renderer/IanStage.test.tsx`：通过，4 个测试文件 / 37 个测试。
- 全量：`desktop:test` / `desktop:typecheck` / `desktop:build` 均通过。

### 桌面 / 预览验收

- 浏览器预览打开设置后确认出现 `宠物预览`，包含“切换到小冒险家 / 小狗 / 小猫 / 羊驼”，当前小狗显示 pressed 状态。
- 真实 Tauri shell 启动 smoke 通过。

### 剩余风险

- 浏览器预览确认了 UI 结构；真实桌面内点击切换未逐项人工验证。
