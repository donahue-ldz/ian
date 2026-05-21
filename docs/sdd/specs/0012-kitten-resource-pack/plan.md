# Implementation Plan: Kitten Resource Pack

## Spec

`docs/sdd/specs/0012-kitten-resource-pack/spec.md`

## Summary

新建 `ian-kitten` Resource Pack，并把桌面端默认 pet/resource pack 从 `ian-alpaca` 切换到 `ian-kitten`。实现保持现有 Resource Pack renderer、动画 manifest、IanEvent/IanAction 行为闭环不变。

## Steps

1. 写前端 RED 测试：在 `apps/desktop/src/resources/resourceLoader.test.ts` 中增加 `ian-kitten` manifest contract 测试，先断言新的 pack id/species/动画集合和 frame count；在资源文件未创建前该测试应失败。
2. 写 Rust RED 测试：把 `apps/desktop/src-tauri/src/storage/config.rs` 中默认恢复断言从 `ian-alpaca` 改为 `ian-kitten`；把 `apps/desktop/src-tauri/src/resources/resource_registry.rs` 的默认 active pet 断言改为 `ian-kitten`。实现未改前测试应失败。
3. 新建 `apps/desktop/public/resources/pets/ian-kitten/pet.json`，声明 `id: "ian-kitten"`、`species: "cat"`、`sprite: "sprite.svg"`、`animations: "animations.json"`、`expressions: "expressions.json"`。
4. 新建 `apps/desktop/public/resources/pets/ian-kitten/animations.json`，沿用当前 96x96、scale 2、14 帧布局，保持 `idle`、`walk`、`sleep`、`run`、`happy` frame 映射不变。
5. 新建 `apps/desktop/public/resources/pets/ian-kitten/expressions.json`，保持当前最小 expression contract。
6. 新建 `apps/desktop/public/resources/pets/ian-kitten/sprite.svg`，绘制 14 帧 Q 版灰白小猫 sprite sheet：
   - idle 2 帧：坐姿、轻微呼吸或眨眼。
   - walk 4 帧：爪子和身体小幅位移。
   - sleep 2 帧：闭眼放松。
   - run 4 帧：身体倾斜、尾巴和爪子动势增强。
   - happy 2 帧：笑眼、脸颊、轻微跳起。
7. 更新默认 pet/resource pack：
   - `apps/desktop/src/App.tsx` 初始加载默认 pack 改为 `ian-kitten`，并优先使用 `getIanSettings()` 返回的 `active_resource_pack` 加载。
   - `apps/desktop/src/lib/tauriBridge.ts` 浏览器 fallback 默认值改为 `ian-kitten`。
   - `apps/desktop/src-tauri/src/protocol/state.rs` `IanState::default()` 改为 `ian-kitten`。
   - `apps/desktop/src-tauri/src/resources/resource_registry.rs` 空 active pet fallback 改为 `ian-kitten`。
8. 运行前端和 Rust 测试，修正只与本任务相关的失败。
9. 启动本地预览，用浏览器验证默认 sprite 来源、idle/happy/run 变化和 bubble 不被遮挡。
10. 更新 `docs/sdd/specs/0012-kitten-resource-pack/decisions.md` 和 `verification.md`，记录实现取舍、命令和结果。
11. 修正桌面拖动链路：保留浏览器预览的内部 CSS 位移，但桌面 Tauri 窗口优先请求原生 `startDragging()`，并使用 `screenX/screenY` 计算屏幕空间位移作为兜底，再调用原生窗口位置 API 移动窗口；增加拖动坐标换算和 capability 回归测试。

## Expected File Changes

- `docs/sdd/specs/0012-kitten-resource-pack/spec.md`: 本规格。
- `docs/sdd/specs/0012-kitten-resource-pack/plan.md`: 本计划。
- `docs/sdd/specs/0012-kitten-resource-pack/decisions.md`: 记录新建资源包和默认切换决策。
- `docs/sdd/specs/0012-kitten-resource-pack/verification.md`: 记录实际验证结果。
- `apps/desktop/public/resources/pets/ian-kitten/pet.json`: 新小猫资源包 manifest。
- `apps/desktop/public/resources/pets/ian-kitten/animations.json`: 新小猫动画 manifest。
- `apps/desktop/public/resources/pets/ian-kitten/expressions.json`: 新小猫表情 manifest。
- `apps/desktop/public/resources/pets/ian-kitten/sprite.svg`: 新小猫 14 帧 sprite sheet。
- `apps/desktop/src/App.tsx`: 默认加载 `ian-kitten`，并使用设置状态中的 active resource pack。
- `apps/desktop/src/lib/tauriBridge.ts`: 浏览器 fallback 默认 pet/resource pack。
- `apps/desktop/src-tauri/src/protocol/state.rs`: Rust 默认 `IanState`。
- `apps/desktop/src-tauri/src/resources/resource_registry.rs`: 默认 active pet fallback 和测试。
- `apps/desktop/src-tauri/src/storage/config.rs`: 默认配置恢复测试。
- `apps/desktop/src/resources/resourceLoader.test.ts`: 新资源包 contract 测试。
- `apps/desktop/src/renderer/dragGesture.ts`: 桌面拖动坐标换算 helper。
- `apps/desktop/src/renderer/IanStage.tsx`: 桌面拖动使用屏幕坐标位移，浏览器预览继续使用 client 坐标位移。
- `apps/desktop/src-tauri/capabilities/default.json`: 允许 Tauri 窗口原生拖动 API。

## Interfaces and Boundaries

- Resource Pack contract 不变：前端仍通过 `loadPetResourcePack(id)` 读取静态 manifest 和 sprite。
- `IanSprite` 渲染接口不变：仍接收 `PetResourcePack | null` 和当前 animation 名称。
- Rust 协议字段不变：只更新默认字符串值，不新增 `IanEvent`、`IanAction` 或 `IanState` 字段。
- Storage 不新增 migration：现有用户配置仍可保存旧 active pet；新默认只影响首次启动、fallback 和损坏配置恢复。

## Verification Commands

```bash
npm --workspace apps/desktop test -- --run src/resources/resourceLoader.test.ts src/renderer/AnimationPlayer.test.ts
cd apps/desktop/src-tauri && cargo test config::tests resource_registry::tests
npm --workspace apps/desktop run dev -- --host 127.0.0.1
```

浏览器手动验证：

- `.ian-sprite-frame` 的 `backgroundImage` 包含 `/resources/pets/ian-kitten/sprite.svg`。
- 初始 `data-animation` 为 `idle`。
- 点击 Ian 后 bubble 可见，`data-animation` 变为 `happy`。
- 双击 Ian 后 `data-animation` 变为 `run`。

## Risks

- 现有本地 `~/.ian/config.toml` 可能仍指定 `ian-alpaca`，导致已安装用户继续看到旧资源包。缓解：本任务明确不做迁移；验证新安装/浏览器 fallback/default state 路径。
- SVG sprite 仍是代码绘制，不是正式美术 bitmap。缓解：本任务按参考图风格制作清晰 Q 版形象，后续可在同一 `ian-kitten` pack 下替换为 PNG sprite sheet。
- 当前工作树有大量未提交改动。缓解：只修改本计划列出的文件，不回滚或整理已有改动。

## Rollback Notes

如需回滚本任务，删除 `apps/desktop/public/resources/pets/ian-kitten`，并把 `App.tsx`、`tauriBridge.ts`、`state.rs`、`resource_registry.rs`、`config.rs` 和相关测试中的默认 id 改回 `ian-alpaca`。`ian-alpaca` 资源包会保留，因此回滚不需要恢复旧美术资源。
