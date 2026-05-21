# Spec: Pet Size Controller

## Status

已实现，待用户验收。

## Context

用户反馈设置面“大小”控件现在只能缩小一下，之后不容易继续操作。当前实现是在高级设置里使用一个窄的原生 `input[type="range"]`，与标签同排放在 `.ian-settings-toggle-row` 中。桌面设置面宽度很小，这让滑块可点击区域和拖动反馈都不够清晰。

相关上下文：

- `apps/desktop/src/renderer/SettingsPanel.tsx`
- `apps/desktop/src/renderer/ianStage.css`
- `apps/desktop/src/App.tsx`
- `apps/desktop/src/lib/tauriBridge.ts`
- `apps/desktop/src-tauri/src/core/creature_state.rs`

## Goal

把 Ian 的大小设置改成可左右互动的紧凑控制器：左侧减号、中间滑条、右侧加号，并显示当前百分比。用户可以连续变小或变大，不需要精确拖动很窄的系统滑块。

## Current Stage

P0 / MVP + v0.1 Architecture Baseline。该工作属于设置体验修复，不新增 Ian 行为能力。

## Product Scope

- 在高级设置中替换现有“大小”原生滑块呈现。
- 控制器包含：
  - 左侧 `-` 按钮，每次减小 `0.1`。
  - 中间 `range` 滑条，支持拖动。
  - 右侧 `+` 按钮，每次增大 `0.1`。
  - 当前百分比显示，例如 `100%`。
- 继续使用现有 `surfaceScale`，范围保持 `0.8` 到 `1.4`，步进保持 `0.1`。
- 到达最小值时禁用减号，到达最大值时禁用加号。
- 设置保存路径继续走现有 `onCreatureSettingsChange` / `saveCreatureSettings`。

## Non-Goals

- 不新增协议字段、Rust command、存储字段或 migration。
- 不改变 `surface_scale` 的范围和 Rust clamp 规则。
- 不改变 Ian 默认大小。
- 不做资源包独立尺寸、皮肤系统、窗口尺寸自适应或多宠物尺寸策略。
- 不重做整个设置面视觉结构。

## User Experience

用户打开设置的高级区后，看到“大小”一行变成一个明确的尺寸控制器。点击 `-` 会让 Ian 逐步变小，点击 `+` 会逐步变大，拖动中间滑条也能连续调整。右侧百分比让用户知道当前值。控件在窄设置面中不挤压文字，不像开关，也不会只能操作一次。

## Architecture Constraints

- React 只负责设置控件呈现和调用既有保存回调。
- Rust Core 继续通过 `IanState.surface_scale` 和现有 config 持久化做单一状态来源。
- 不新增 `IanEvent` / `IanAction`，因为大小控制是设置写入，不是 Ian 行为决策。
- 设置面必须保持本地优先，不读取任何外部信息。

## Data and Protocol Changes

None。

继续使用现有：

- TypeScript prop：`surfaceScale`
- Tauri bridge payload：`surface_scale`
- Rust state field：`IanState.surface_scale`
- Rust clamp：`0.8..=1.4`

## Privacy and Security

无新增权限、外部输入、网络访问或敏感数据读取。该控件只写入本地用户设置。

## Acceptance Criteria

- [ ] 高级设置中“大小”控件渲染为 `-` 按钮、滑条、`+` 按钮和当前百分比。
- [ ] 点击 `+` 时，`surfaceScale` 以 `0.1` 为步进增加，最大不超过 `1.4`。
- [ ] 点击 `-` 时，`surfaceScale` 以 `0.1` 为步进减少，最小不低于 `0.8`。
- [ ] 当前值为 `0.8` 时减号禁用，当前值为 `1.4` 时加号禁用。
- [ ] 拖动滑条仍调用现有 `onCreatureSettingsChange` 并传递新的 `surfaceScale`。
- [ ] 控制器在设置面窄宽度下不会被误呈现为开关，不会与“本地诊断”行重叠。
- [ ] 不新增或修改协议、Rust state 字段、存储字段或 migration。

## Verification Approach

- `npm run desktop:test -- SettingsPanel.view.test.tsx`
- `npm run desktop:test`
- `npm run desktop:typecheck`
- Browser smoke：打开设置高级区，确认大小控制器可见；点击 `+` / `-` 后百分比变化，滑条仍可拖动。

## Open Questions

None。
