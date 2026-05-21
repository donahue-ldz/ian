# 实现计划: Resource Pack Renderer

## Spec

`docs/sdd/specs/0002-resource-pack-renderer/spec.md`

## 状态

已实现，待最终收尾。

## 概要

将 0001 的 CSS placeholder creature 替换为 Resource Pack 驱动的 sprite-sheet renderer：

- 增强 `AnimationPlayer` 的帧选择逻辑。
- 添加动画帧选择测试。
- 让 `IanSprite` 使用 `pet.json.sprite` 和 `animations.json` 渲染帧。
- 将默认 `sprite.svg` 调整为 sprite sheet 风格资源。

## 验收标准映射

| 实现步骤 | 对应验收标准 |
| --- | --- |
| 1. 添加 animation frame 测试 | frame selection、fallback、测试覆盖 |
| 2. 实现 `AnimationPlayer` 纯函数 | manifest frames/fps/loop、fallback idle |
| 3. 实现 sprite sheet asset | idle/happy/run 不同 frame、resource pack 路径 |
| 4. 改造 `IanSprite` | 正常路径使用 Resource Pack sprite asset |
| 5. 验证 | npm test/typecheck/build、cargo check |

## 步骤

1. 添加 TDD 失败测试。
   - 状态：完成。
   - 修改 `apps/desktop/src/renderer/AnimationPlayer.test.ts`。
   - 覆盖：
     - `resolveAnimation` 对未知动画 fallback 到 `idle`。
     - `resolveAnimationFrame` 按 elapsed 和 fps 选择 frame。
     - 非 loop animation 停在最后一帧。

2. 实现 animation frame 纯函数。
   - 状态：完成。
   - 修改 `apps/desktop/src/renderer/AnimationPlayer.ts`。
   - 添加：
     - `resolveAnimationDefinition`
     - `resolveAnimationFrame`
     - `getSpriteSheetFrameCount`
   - 保持无 React 依赖，便于测试。

3. 改造 `IanSprite`。
   - 状态：完成。
   - 修改 `apps/desktop/src/renderer/IanSprite.tsx`。
   - 从 Resource Pack 读取 sprite URL、frame meta、animation definition。
   - 使用 CSS background-position 显示当前帧。
   - 只有 `resourcePack` 缺失时使用 CSS fallback creature。

4. 调整样式。
   - 状态：完成。
   - 修改 `apps/desktop/src/renderer/ianStage.css`。
   - 添加 `.ian-sprite-frame` 的稳定尺寸、pixelated rendering、背景定位。
   - 保留 fallback CSS creature，但只在 fallback class 下显示。

5. 调整默认资源。
   - 状态：完成。
   - 修改 `apps/desktop/public/resources/pets/ian-alpaca/sprite.svg`。
   - 让 SVG 宽度为 `frameWidth * frameCount`，高度为 `frameHeight`。
   - 生成至少 14 个可区分 frame，满足现有 `animations.json`。

6. 验证并记录。
   - 状态：完成。
   - 运行 `npm run desktop:test`。
   - 运行 `npm run desktop:typecheck`。
   - 运行 `npm run desktop:build`。
   - 运行 `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`。
   - 更新 `verification.md`。

## 预计文件改动

- `apps/desktop/src/renderer/AnimationPlayer.ts`
- `apps/desktop/src/renderer/AnimationPlayer.test.ts`
- `apps/desktop/src/renderer/IanSprite.tsx`
- `apps/desktop/src/renderer/ianStage.css`
- `apps/desktop/src/resources/resourceLoader.ts`
- `apps/desktop/public/resources/pets/ian-alpaca/sprite.svg`
- `docs/sdd/specs/0002-resource-pack-renderer/decisions.md`
- `docs/sdd/specs/0002-resource-pack-renderer/verification.md`

## 接口与边界

- 不修改 Rust `IanEvent` / `IanAction` / `IanState`。
- `IanSprite` 只根据 view state 执行动画，不决定行为。
- Resource Pack manifest 继续作为渲染资源描述来源。

## 验证命令

```bash
npm run desktop:test
npm run desktop:typecheck
npm run desktop:build
cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml
```

## 风险

- SVG sprite sheet 是 placeholder，视觉质量有限。
- React interval 动画需要避免过度重渲染；P0 帧数很少，风险可接受。
- 后续如果接入真实 PNG sprite sheet，需要确认 frameWidth/frameHeight 和 asset 尺寸一致。

## 回滚说明

- 回滚 `IanSprite` 和 `AnimationPlayer` 改动后，会回到 0001 CSS fallback creature。
- Resource Pack manifest 不需要回滚。
