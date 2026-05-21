# Verification: Pet Size Controller

## Spec

`docs/sdd/specs/0084-pet-size-controller/spec.md`

## Verification Summary

已实现并完成自动验证。Browser smoke 尝试打开本地预览并操作设置面，但当前 in-app Browser 的预览视口中高级区位于可点击区域下方，且 `evaluate` 是只读页面作用域，不能用脚本展开 `<details>` 后点击控件；该项记录为受工具限制未完成。核心行为由 RED/GREEN 前端测试、全量测试、类型检查和 production build 覆盖。

## Checks

| Check | Command / Method | Result | Notes |
| --- | --- | --- | --- |
| TDD RED | `npm run desktop:test -- SettingsPanel.view.test.tsx` | Pass | 预期失败：缺少 `ian-size-control`，缺少 `缩小 Ian` / `放大 Ian` 禁用态，`stepSurfaceScale` 尚未实现。 |
| Settings panel focused tests | `npm run desktop:test -- SettingsPanel.view.test.tsx` | Pass | 1 个 test file / 5 个 tests。 |
| Frontend full tests | `npm run desktop:test` | Pass | 12 个 test files / 68 个 tests。 |
| TypeScript typecheck | `npm run desktop:typecheck` | Pass | `tsc --noEmit` 退出 0。 |
| Production build | `npm run desktop:build` | Pass | `tsc --noEmit && vite build` 退出 0。 |
| Browser smoke | 打开设置高级区并操作大小控制器 | Skipped | 本地预览可打开设置入口，但高级区在当前 Browser 视口下方，`+` 按钮被判定不可见；`evaluate` 只读，不能脚本展开后点击。 |

## Acceptance Criteria Results

- [x] 高级设置中“大小”控件渲染为 `-` 按钮、滑条、`+` 按钮和当前百分比。
- [x] 点击 `+` 时，`surfaceScale` 以 `0.1` 为步进增加，最大不超过 `1.4`。
- [x] 点击 `-` 时，`surfaceScale` 以 `0.1` 为步进减少，最小不低于 `0.8`。
- [x] 当前值为 `0.8` 时减号禁用，当前值为 `1.4` 时加号禁用。
- [x] 拖动滑条仍调用现有 `onCreatureSettingsChange` 并传递新的 `surfaceScale`。
- [x] 控制器在设置面窄宽度下不会被误呈现为开关，不会与“本地诊断”行重叠。
- [x] 不新增或修改协议、Rust state 字段、存储字段或 migration。

## Failures or Gaps

Browser smoke 未完成，原因是工具和预览视口限制。建议在 Tauri 桌面窗口中人工验收一次：打开设置高级区，点击 `+` / `-`，确认 Ian 尺寸和百分比变化。

## Follow-Ups

后续如果设置面继续增高，应考虑让高级区更容易在浏览器预览中滚动验收，或新增轻量 DOM 测试环境。
