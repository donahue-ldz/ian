# 验证记录: Resource Pack Renderer

## Spec

`docs/sdd/specs/0002-resource-pack-renderer/spec.md`

## 状态

已验证。

## 验证摘要

0002 已把可见 Ian sprite 从 CSS placeholder 正常路径切到 Resource Pack sprite sheet renderer。Animation frame selection 有单元测试覆盖；本地浏览器预览确认 DOM 使用 `.ian-sprite-frame` 和 resource pack sprite asset，idle 帧会推进，双击后进入 `run` 帧。

## 检查项

| 检查项 | 命令 / 方法 | 结果 | 备注 |
| --- | --- | --- | --- |
| TDD RED | `npm run desktop:test` | 失败符合预期 | 新增测试先失败，缺少 `resolveAnimationFrame` 和 `getSpriteSheetFrameCount`。 |
| Frontend test | `npm run desktop:test` | 通过 | 2 个测试文件，6 个测试通过。 |
| Frontend typecheck | `npm run desktop:typecheck` | 通过 | `tsc --noEmit` 通过。 |
| Frontend build | `npm run desktop:build` | 通过 | `tsc --noEmit && vite build` 通过。 |
| Rust fmt | `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml --check` | 通过 | Rust 文件格式未变化。 |
| Rust check | `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` | 通过 | Tauri app 仍可编译。 |
| Browser preview | `http://127.0.0.1:5175/` | 通过 | `.ian-sprite-frame` 使用 `resources/pets/ian-alpaca/sprite.svg`；idle frame 位置在 `0px` 与 `-192px` 间推进；双击后 `data-animation="run"` 且背景定位到 run frame。 |
| Bubble layering regression | `http://localhost:1420/` 点击 Ian 后检查 DOM/CSS | 通过 | 气泡文本为“我在这儿。”；`.ian-bubble` 为 `z-index: 2`，`.ian-click-target` 为 `z-index: 1`，气泡不再被 sprite 覆盖。 |
| Run-around expiry regression | `http://localhost:1420/` 双击 Ian 后等待 | 通过 | 双击后 150ms 为 `data-animation="run"`；约 2.1s 后自动回到 `data-animation="idle"`。 |

## 验收标准结果

- [x] `IanSprite` 正常路径使用 Resource Pack sprite asset 渲染，而不是 CSS 拼图。
- [x] `AnimationPlayer` 或等价模块根据 manifest 的 `frames`、`fps`、`loop` 计算当前帧。
- [x] 请求不存在的 animation 时 fallback 到 `idle`。
- [x] `idle`、`happy`、`run` 至少能映射到不同 sprite sheet frame。
- [x] Resource Pack manifest 仍位于 `apps/desktop/public/resources/pets/ian-alpaca`。
- [x] 前端测试覆盖 animation frame selection。
- [x] `npm run desktop:test`、`npm run desktop:typecheck`、`npm run desktop:build` 通过。
- [x] `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` 通过。
- [x] 点击气泡在 sprite sheet 渲染路径下可见。
- [x] 双击 run-around 在 `duration_ms` 后自动回到 idle。

## 失败或缺口

暂无阻塞。

## 后续

- 真实美术资源接入后，需要再次校验 frame 尺寸和 manifest 是否一致。
