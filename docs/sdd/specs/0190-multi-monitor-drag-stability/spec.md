# Spec: 多显示器拖动稳定性

## Status

Draft

## Context

用户反馈 Ian 在连接多个显示器时拖动会有问题。当前环境包含三块显示器：内置 Retina 屏、HP 4K 外接屏和 DELL 1080p 外接屏。现有实现中，桌面端拖动同时调用 Tauri 原生 `startDragging()`，并在 pointer move 中根据 `event.screenX/screenY`、`window.devicePixelRatio` 和初始窗口位置手动调用 `setPosition()`。这会让原生拖动和前端手动移动同时控制同一个 Tauri 窗口，在跨显示器或混合缩放环境中容易出现跳动、偏移或拖动终点错误。

相关代码：

- `apps/desktop/src/renderer/IanStage.tsx`
- `apps/desktop/src/App.tsx`
- `apps/desktop/src/lib/position.ts`
- `apps/desktop/src/renderer/dragGesture.ts`
- `apps/desktop/src-tauri/capabilities/default.json`

## Goal

让桌面端 Ian 拖动只使用一条稳定的窗口移动路径：桌面端交给 Tauri 原生窗口拖动，拖动结束后读取真实 `outerPosition()` 并持久化，避免前端在多显示器坐标系中自行计算窗口位置。

## Current Stage

P0 / MVP + v0.1 Architecture Baseline

## Product Scope

- 桌面端拖动 Ian 时，窗口跟随系统原生拖动行为移动。
- 拖动结束后，Rust Core 仍收到 `mouse.drag_end`，并保存真实窗口位置。
- 浏览器预览拖动行为保持现有 CSS transform 预览方式。
- 点击、双击、气泡、动画和设置面板不因本修复改变。

## Non-Goals

- 不实现跨显示器自动漫游。
- 不实现多显示器安全边界或停靠策略。
- 不改变 `IanEvent` / `IanAction` / `IanState` 协议。
- 不新增桌面权限。
- 不读取窗口标题、屏幕内容、应用内容、剪贴板或代码内容。

## User Experience

用户在任意显示器上按住 Ian 并拖动时，Ian 窗口应跟随鼠标移动，不出现明显跳动或与鼠标偏离。松开鼠标后，Ian 保持在释放位置；重启后从保存的位置恢复。

## Architecture Constraints

- React 只负责识别拖动手势、触发 Tauri 原生拖动、发送 `mouse.drag_start` / `mouse.drag_end`，不成为桌面窗口坐标策略的行为大脑。
- 桌面窗口真实位置以 Tauri `outerPosition()` 为准。
- Rust Core 继续通过 `save_window_position` 保存位置并同步 `IanState`。
- 浏览器预览仍可使用 React 本地偏移，不影响真实桌面窗口。

## Data and Protocol Changes

None.

## Privacy and Security

本修复仅使用已有 Tauri window 权限：

- `core:window:allow-start-dragging`
- `core:window:allow-outer-position`

不新增敏感输入，不读取显示器内容或窗口标题。拖动结束保存的仅是窗口坐标。

## Acceptance Criteria

- [ ] 桌面端拖动开始后，只调用 Tauri 原生 `startDragging()` 作为窗口移动机制，不再在 pointer move 中同时调用 `setPosition()`。
- [ ] 桌面端拖动结束后，使用 `getCurrentWindow().outerPosition()` 的真实窗口位置发送 `mouse.drag_end` 并调用 `save_window_position`。
- [ ] 浏览器预览拖动仍使用本地 `dragOffset`，相关测试保持通过。
- [ ] 现有 Tauri window capability 不新增权限。
- [ ] 在真实 Tauri 桌面壳中，至少在当前三屏环境下手动验证：从一个显示器拖动到另一个显示器时窗口跟随鼠标，释放后位置不跳回，重启后位置恢复。

## Verification Approach

- 运行定向前端测试：

```bash
PATH="/opt/homebrew/bin:/usr/local/bin:$HOME/.cargo/bin:$PATH" npm run desktop:test -- --run src/renderer/dragGesture.test.ts src/lib/position.test.ts src/renderer/IanStage.test.tsx
```

- 运行桌面端 Rust 测试：

```bash
PATH="/opt/homebrew/bin:/usr/local/bin:$HOME/.cargo/bin:$PATH" cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
```

- 启动真实桌面壳：

```bash
./scripts/start-desktop.sh
```

- 手动在三屏环境拖动 Ian，记录是否稳定以及重启后位置是否恢复。

## Open Questions

- 用户观察到的问题具体表现可能是跳动、偏移、拖不过屏幕边界或释放后回弹。本 spec 先修复代码中已确认的双重窗口移动根因；如果桌面验收仍复现其他症状，再追加新的诊断记录和范围变更。
