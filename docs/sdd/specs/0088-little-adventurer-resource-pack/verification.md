# 0088 验证记录

## 2026-05-21

- `PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:test -- resourceLoader.test.ts`
  - 结果：通过，1 个测试文件，6 个测试。
- `qlmanage -t -s 3456 -o /tmp apps/desktop/public/resources/pets/ian-adventurer/sprite.svg`
  - 结果：通过，生成预览 `/tmp/sprite.svg.png`。
- 视觉迭代记录：
  - 初版彩色萌宠方向不符合用户参考。
  - 白色 / 极浅空心线稿方向也不符合用户参考。
  - 黑灰剪影方向太黑，桌面上容易糊成一团。
  - 当前版本调整为 4px 网格像素风：灰蓝主体、浅色脸部、青色围巾、黄色发光探路棒。

## 待补充

- 仍需在真实 Tauri 桌面壳里验证默认资源包加载、透明窗口边界、拖动和追鼠标时的观感。
- 当前工作树存在其他会话遗留的 `ianActions.test.ts` 失败，完整测试暂不能作为 0088 的通过依据。
