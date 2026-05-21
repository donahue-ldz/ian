# Verification: Resource Pack Visual Quality Pass

## 状态

已验证。

## 需要记录的验证

- [x] 资源 manifest 校验
- [x] `npm run desktop:test`
- [x] `npm run desktop:typecheck`
- [x] 主要状态截图：idle、walk、run、happy、rest、sleep
- [x] fallback 场景检查

## 结果

### TDD RED

- `npm run desktop:test -- resourceLoader.test.ts`
  - 初始失败：测试正则未正确读取 SVG width，修正测试后重新 RED。
- `npm run desktop:test -- resourceLoader.test.ts`
  - 预期失败：`rest` 帧为 `0,1`，和 `idle` 完全相同。

### 自动验证

- `npm run desktop:test`
  - 通过：9 files / 38 tests。
- `npm run desktop:typecheck`
  - 通过。
- `npm run desktop:build`
  - 通过，Vite build 完成。

### Resource Smoke

- 默认资源包检查覆盖：`ian-alpaca`、`ian-kitten`。
- 关键动画：`idle`、`walk`、`run`、`happy`、`rest`、`sleep`。
- 校验项：
  - capabilities 包含关键动画。
  - 关键动画均有帧。
  - 帧号没有超过 SVG sprite sheet width。
  - `rest` / `sleep` 不复用 `idle` 的完整帧集合。
  - unknown animation fallback 到 `idle`。

### Browser Screenshot

- 预览页：`resource-preview.html`
- 截图：`screenshots/resource-states-light-dark.jpg`
- 浏览器检查结果：
  - `frameCount: 12`
  - captions 覆盖浅色和深色背景下的 `idle`、`walk`、`run`、`happy`、`rest`、`sleep`
  - sprite background image 已加载。

### 结论

0059 验收标准已覆盖。默认资源包具备独立 `rest` 帧，并保留 resource fallback 行为。
