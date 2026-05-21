# 实现计划: Presence Surface Polish

## 对应规格

`docs/sdd/specs/0035-presence-surface-polish/spec.md`

## 实现步骤

1. 为 click / drag / bubble input 冲突写前端回归测试。
2. 调整 IanStage 和 CSS 的命中区域、透明背景和阴影。
3. 如需增加 scale 配置，先写 config / UI 测试。
4. 用 Browser 或 Playwright 截图检查透明表面和气泡。
5. 用 Tauri 本地 smoke 确认窗口行为。

## 预计改动文件

- `apps/desktop/src/renderer/IanStage.tsx`
- `apps/desktop/src/renderer/ianStage.css`
- `apps/desktop/src/renderer/dragGesture.ts`
- `apps/desktop/src/renderer/dragGesture.test.ts`
- `apps/desktop/src-tauri/src/desktop/window.rs`
- `docs/sdd/specs/0035-presence-surface-polish/*`

## 接口 / 兼容性

如果新增 scale 配置，旧配置使用默认 1.0。视觉改动不得改变 Rust 行为协议。

## 验证命令

```bash
npm run desktop:test
npm run desktop:typecheck
npm run desktop:build
```

## 风险和回滚

窗口表面改动依赖平台行为。回滚方式是保留交互测试，撤回阴影或高级透明样式。
