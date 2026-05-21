# Implementation Plan: Pet Size Controller

## Spec

`docs/sdd/specs/0084-pet-size-controller/spec.md`

## Summary

把 `SettingsPanel` 中高级设置的“大小”控件从窄原生滑块替换为紧凑尺寸控制器。实现只触碰 React 设置面、CSS、前端测试和本 SDD packet，不改 Rust 协议、状态或存储。

## Steps

1. 为 `SettingsPanel.view.test.tsx` 增加失败测试，断言大小控制器渲染 `-`、`+`、百分比和滑条。
2. 为 `SettingsPanel.view.test.tsx` 增加纯函数测试，断言 `+` / `-` 的步进、clamp 和百分比格式正确。当前测试环境没有 DOM 测试运行时，不为该控件新增 `jsdom` / `happy-dom` 依赖。
3. 运行 `npm run desktop:test -- SettingsPanel.view.test.tsx`，确认 RED 失败来自缺少新控制器。
4. 在 `SettingsPanel.tsx` 中抽出 `SizeController` 小组件，复用现有 `onCreatureSettingsChange` payload，不新增外部接口。
5. 将高级设置中的“大小”行替换为 `SizeController`。
6. 在 `ianStage.css` 中添加 `.ian-size-control`、`.ian-size-button`、`.ian-size-range`、`.ian-size-value` 样式，保证窄面板中稳定布局。
7. 运行 `npm run desktop:test -- SettingsPanel.view.test.tsx`，确认 GREEN。
8. 运行 `npm run desktop:test` 和 `npm run desktop:typecheck`。
9. Browser smoke：打开设置高级区，手动点击 `+` / `-`，确认百分比和 Ian 尺寸变化。
10. 更新 `verification.md`，记录命令、结果、Browser smoke 和剩余风险。

## Expected File Changes

- `apps/desktop/src/renderer/SettingsPanel.tsx`: 新增 `SizeController`，替换“大小”原生滑块行。
- `apps/desktop/src/renderer/SettingsPanel.view.test.tsx`: 增加渲染和交互回归测试。
- `apps/desktop/src/renderer/ianStage.css`: 增加尺寸控制器样式。
- `docs/sdd/specs/0084-pet-size-controller/spec.md`: 本 spec。
- `docs/sdd/specs/0084-pet-size-controller/plan.md`: 本计划。
- `docs/sdd/specs/0084-pet-size-controller/decisions.md`: 记录尺寸范围和控件形态决策。
- `docs/sdd/specs/0084-pet-size-controller/verification.md`: 记录实际验证。

## Interfaces and Boundaries

- React 设置面仍只调用 `onCreatureSettingsChange`。
- `App.tsx`、`tauriBridge.ts`、Rust Core、config persistence 保持不变。
- `surfaceScale` 的业务范围继续由 Rust `creature_state.rs` clamp 兜底；React 控件也在 UI 层避免越界。
- 不新增 `IanEvent` / `IanAction`。

## Verification Commands

```bash
npm run desktop:test -- SettingsPanel.view.test.tsx
npm run desktop:test
npm run desktop:typecheck
```

## Risks

- 风险：设置面宽度很窄，新控件可能与文字挤压。
  - 缓解：控制器使用固定按钮尺寸、弹性滑条和短百分比显示；测试覆盖结构，Browser smoke 检查视觉。
- 风险：浮点步进出现 `1.2000000000000002`。
  - 缓解：React helper 用一位小数规整后再保存和展示百分比。
- 风险：只更新 UI 控件但未保存。
  - 缓解：纯函数测试覆盖新尺寸值计算；静态渲染测试覆盖 `+` / `-` / range 控件存在，Browser smoke 覆盖真实点击后 UI 更新。

## Rollback Notes

如需回滚，只恢复 `SettingsPanel.tsx` 的原生 `input[type="range"]` 行并删除新增样式和测试。本工作不包含协议、Rust 或存储迁移，回滚不需要数据处理。
