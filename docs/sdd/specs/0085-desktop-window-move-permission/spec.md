# Spec: Desktop Window Move Permission

## Status

Approved

## Context

用户指出自动游走的重点不是浏览器预览是否满足，而是桌面版本是否真实满足。当前自动游走会通过 `IanAction::MovementMoveTo` 到前端 `moveDesktopWindow()`，再调用 Tauri `getCurrentWindow().setPosition(...)` 移动透明桌面窗口；但 `apps/desktop/src-tauri/capabilities/default.json` 只显式允许 `core:window:allow-start-dragging`，没有显式允许 `core:window:allow-set-position`。

## Goal

确保桌面版 Ian 执行 `movement.move_to` 时具备移动原生 Tauri 窗口的权限，并用测试覆盖该权限，避免只在浏览器预览中满足自动游走。

## Current Stage

P0 / MVP + v0.1 Architecture Baseline

## Product Scope

- 保持现有自动游走行为、频率和路径不变。
- 只补齐桌面窗口移动所需 Tauri capability。
- 用前端测试读取 capability 配置，防止桌面移动权限再次缺失。

## Non-Goals

- 不调整自动游走频率、路径、动画或策略。
- 不新增用户可见设置。
- 不新增全局屏幕感知或窗口感知能力。
- 不改动 `IanEvent` / `IanAction` / `IanState` 协议。

## User Experience

在 Tauri 桌面版中，当 Rust Core 输出 `movement.move_to` 时，React 执行层可以调用 Tauri 窗口 API 移动透明 Ian 窗口。浏览器预览仍保持原有 fallback 表现。

## Architecture Constraints

- Rust Core 继续负责行为决策，自动游走目标仍由 Rust Core 产生。
- React 继续只执行 `IanAction`，不生成自动游走目标。
- 桌面窗口移动通过 Tauri window capability 授权，不绕过权限模型。

## Data and Protocol Changes

None.

## Privacy and Security

新增权限仅允许当前 Ian 主窗口设置自身位置，不读取敏感内容，不增加外部输入源，不扩大到全局键盘、OCR、剪贴板或应用内容访问。

## Acceptance Criteria

- [ ] `apps/desktop/src-tauri/capabilities/default.json` 包含 `core:window:allow-set-position`。
- [ ] capability 回归测试在缺少 `core:window:allow-set-position` 时失败。
- [ ] `movement.move_to` 的执行链路仍通过 `moveDesktopWindow()` 调用 Tauri window API，不在 React 中生成行为目标。
- [ ] 相关前端测试通过。

## Verification Approach

```bash
npm run desktop:test -- IanStage.test.tsx
npm run desktop:test -- ianActions.test.ts
```

## Open Questions

- None.
