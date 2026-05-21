# Verification: Pet Resource Pack Switcher

## Spec

`docs/sdd/specs/0089-pet-resource-pack-switcher/spec.md`

## Verification Summary

已完成 RED/GREEN 验证。Rust 先因缺少 `save_active_pet` 失败；前端先因缺少 `switchPetResourcePack` 和设置面板宠物选择器失败。实现后，目标测试、前端全量测试、类型检查和 Rust 全量测试通过。

## Checks

| Check | Command / Method | Result | Notes |
| --- | --- | --- | --- |
| Rust RED active pet command | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml active_pet` | Failed as expected | 编译失败：`IanRuntime` 没有 `save_active_pet`。 |
| Frontend RED switcher | `../../node_modules/.bin/vitest run SettingsPanel.view.test.tsx App.test.ts` from `apps/desktop` | Failed as expected | 3 failed：缺少 `switchPetResourcePack`，设置面板缺少 `aria-label="选择宠物"`。 |
| Rust active pet target | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml active_pet` | Passed | 2 tests passed。 |
| Frontend switcher target | `../../node_modules/.bin/vitest run SettingsPanel.view.test.tsx App.test.ts` from `apps/desktop` | Passed | 2 files / 13 tests passed。 |
| Related frontend regression | `../../node_modules/.bin/vitest run App.test.ts SettingsPanel.view.test.tsx IanStage.test.tsx resourceLoader.test.ts` from `apps/desktop` | Passed | 4 files / 32 tests passed。 |
| Frontend typecheck | `../../node_modules/.bin/tsc --noEmit` from `apps/desktop` | Passed | 退出码为 0。 |
| Full frontend tests | `../../node_modules/.bin/vitest run` from `apps/desktop` | Passed | 12 files / 79 tests passed。 |
| Full Rust tests | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` | Passed | 117 tests passed。 |
| Tauri desktop smoke attempt | Existing `tauri dev` / `ian_desktop` process on port `1420` | Inconclusive | 已有进程运行，未强杀；当前工具无法可靠点击透明桌面窗口完成设置切换，所以未记为通过。 |

## Acceptance Criteria Results

- [x] 设置面板打开时能看到“宠物”选择器，并列出当前内置 resource pack。
- [x] 用户选择一个宠物后，React 调用保存命令，并在保存成功后加载对应 resource pack。
- [x] 保存成功后 `IanState.active_pet_id` 和 `IanState.active_resource_pack` 都更新为选中的 id。
- [x] 下次启动 / 加载设置时，App 按 `state.active_resource_pack` 加载 resource pack。
- [x] 未知宠物 id 会被 Rust command 拒绝，不会写入 state。
- [x] resource pack 加载失败时，前端不保存失败选择，并保留当前已加载资源包。

## Failures or Gaps

真实 Tauri 桌面端视觉切换未完成验收。当前机器已有 `vite` / `ian_desktop` dev 进程占用 `1420`，为避免干扰用户正在运行的桌面进程，本轮没有强杀重启；工具也无法可靠点击透明窗口设置菜单。仍需人工在当前桌面壳中打开设置，切换“宠物”，确认外观立即变化并重启后保留选择。
