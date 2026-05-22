# Implementation Plan: 多显示器拖动稳定性

## Spec

`docs/sdd/specs/0190-multi-monitor-drag-stability/spec.md`

## Summary

移除桌面端 pointer move 中的手动窗口位移路径，让桌面端拖动只调用 Tauri `startDragging()`；拖动结束后继续读取 `outerPosition()`，把真实窗口位置交给 Rust Core 保存。浏览器预览保留现有 `dragOffset`。

## Steps

1. 更新 `apps/desktop/src/renderer/dragGesture.test.ts`，把 `getPhysicalDragOffset` 的测试替换为桌面端不需要前端物理坐标换算的断言，先得到失败测试。
2. 更新 `apps/desktop/src/renderer/dragGesture.ts`，删除 `getPhysicalDragOffset`，保留 `getDragOffset` 和 `shouldStartDrag`。
3. 更新 `apps/desktop/src/renderer/IanStage.tsx`，删除 `dragScreenOrigin`、`screenPointFromPointer` 和桌面端 pointer move 的 `onDragMove(getPhysicalDragOffset(...))`；桌面端开始拖动后只触发 `onDragStart` 和 `startDragging()`，浏览器预览继续使用 `setDragOffset(offset)`。
4. 更新 `apps/desktop/src/App.tsx`，删除 `desktopDragOrigin` 与 `onDragMove` 中的 `moveDesktopWindow` 手动移动逻辑；保留 `onDragEnd` 中读取 `getDesktopWindowPosition()` 并保存真实位置。
5. 清理未使用 import：`IanStage.tsx` 不再导入 `getPhysicalDragOffset`，`App.tsx` 不再导入或调用 `moveDesktopWindow` 作为拖动路径。
6. 运行定向前端测试和 Rust 测试。
7. 启动真实 Tauri 桌面壳，执行多显示器拖动验收，并把命令、结果、失败或风险写入 `verification.md`。

## Expected File Changes

- `apps/desktop/src/renderer/dragGesture.ts`: 删除桌面端物理偏移换算函数。
- `apps/desktop/src/renderer/dragGesture.test.ts`: 调整拖动手势测试，覆盖浏览器预览偏移和拖动阈值，不再鼓励桌面端手动物理坐标换算。
- `apps/desktop/src/renderer/IanStage.tsx`: 桌面端 pointer move 不再手动移动窗口；浏览器预览保持 transform 拖动。
- `apps/desktop/src/App.tsx`: 删除桌面拖动的手动 `setPosition` 起点和移动逻辑。
- `docs/sdd/specs/0190-multi-monitor-drag-stability/verification.md`: 记录验证结果。

## Interfaces and Boundaries

- `IanStage` 继续通过 `onDragStart` / `onDragEnd` 与上层通信。
- `App` 继续在拖动开始发送 `mouse.drag_start`，在拖动结束发送 `mouse.drag_end` 并保存位置。
- 桌面窗口移动由 Tauri window API 负责，React 不再计算跨显示器物理坐标。
- Rust Core、协议类型、持久化格式和 capability 不改变。

## Verification Commands

```bash
PATH="/opt/homebrew/bin:/usr/local/bin:$HOME/.cargo/bin:$PATH" npm run desktop:test -- --run src/renderer/dragGesture.test.ts src/lib/position.test.ts src/renderer/IanStage.test.tsx
PATH="/opt/homebrew/bin:/usr/local/bin:$HOME/.cargo/bin:$PATH" cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
./scripts/start-desktop.sh
```

## Risks

- Tauri 原生 `startDragging()` 在无装饰透明窗口上的行为受平台限制；如果真实桌面验收仍有问题，需要补充 `onMoved` 监听或 Rust 侧诊断，而不是重新叠加手动移动。
- 现有全量前端测试当前已有一个与设置面板“记忆候选”文案相关的失败，可能与本修复无关；验证记录必须明确区分。

## Rollback Notes

如果修复导致桌面端无法拖动，可回退 `IanStage.tsx` 和 `App.tsx` 中本 packet 的改动，恢复 `getPhysicalDragOffset` 和 `moveDesktopWindow` 拖动路径；回退后仍需重新验证多显示器问题。
