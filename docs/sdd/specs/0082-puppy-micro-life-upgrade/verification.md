# Verification: Puppy Resource Pack + Micro Life Upgrade

对应 spec：`docs/sdd/specs/0082-puppy-micro-life-upgrade/spec.md`

## 状态

已实现；已处理第一轮视觉和速度验收反馈。

## 验证命令

| 类型 | 命令 | 结果 | 说明 |
| --- | --- | --- | --- |
| TDD RED | `npm run desktop:test -- resourceLoader.test.ts` | 失败符合预期 | `ian-puppy/pet.json` 不存在。 |
| Resource / animation | `npm run desktop:test -- resourceLoader.test.ts AnimationPlayer.test.ts` | 通过 | 10 tests passed。 |
| Storage RED | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml storage` | 失败符合预期 | 默认 state 仍是 `ian-alpaca`。 |
| Storage | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml storage` | 通过 | 9 tests passed；新默认 puppy，旧 config 保留 alpaca。 |
| Behavior RED | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior` | 失败符合预期 | 缺少 `tail_wag`、settle、recent wake phrase。 |
| Behavior | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior` | 通过 | 49 tests passed。 |
| Frontend CSS RED | `npm run desktop:test -- IanStage.test.tsx` | 失败符合预期 | 缺少 `tail_wag` CSS。 |
| Frontend subset | `npm run desktop:test -- resourceLoader.test.ts AnimationPlayer.test.ts ianActions.test.ts IanStage.test.tsx` | 通过 | 31 tests passed。 |
| Frontend all | `npm run desktop:test` | 通过 | 12 files / 64 tests passed。 |
| Rust protocol | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml protocol` | 通过 | 17 tests passed。 |
| Rust all | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` | 通过 | 107 tests passed。 |
| Typecheck | `npm run desktop:typecheck` | 通过 | `tsc --noEmit` passed。 |
| Build | `npm run desktop:build` | 通过 | Vite production build completed. |
| Review RED | `npm run desktop:test -- resourceLoader.test.ts ianActions.test.ts IanStage.test.tsx` | 失败符合预期 | 新增验收反馈测试后，puppy FPS、移动节奏、CSS loop 和 `rounded-puppy-v2` 标记尚未满足。 |
| Review subset | `npm run desktop:test -- resourceLoader.test.ts ianActions.test.ts IanStage.test.tsx` | 通过 | 31 tests passed；覆盖 puppy 降速、移动节奏变化和高能 CSS loop。 |
| Review frontend all | `npm run desktop:test` | 通过 | 12 files / 72 tests passed。 |
| Review Rust all | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` | 通过 | 107 tests passed。 |
| Review typecheck | `npm run desktop:typecheck` | 通过 | `tsc --noEmit` passed。 |
| Review build | `npm run desktop:build` | 通过 | Vite production build completed。 |

## Build Notes

`npm run desktop:build` initially surfaced pre-existing untracked-test integration gaps in `SettingsPanel.view.test.tsx` and `App.test.ts`. The final build passed after small compatibility repairs already present in the working tree:

- `SettingsPanel` exports `stepSurfaceScale` and `formatSurfaceScalePercent` from the existing size-controller implementation.
- `App` exposes `sendArmedChaseCandidate`, matching the existing pointer-chase test helper expectation.

## Browser / Desktop Smoke

Browser smoke used the existing local Vite server at `http://localhost:1420/`:

- 默认 sprite background URL includes `/resources/pets/ian-puppy/sprite.svg`.
- Direct `time.tick` at `now_ms: 30000` returns `effect.play tail_wag` with `intensity: low`.
- Direct `mouse.double_click` returns `playful.state` with `state: settling` and `effect.play blush_puff`.
- Browser console/page errors: none.

Review smoke after visual/speed feedback:

- Direct SVG sprite render confirmed nonblank `rounded-puppy-v2` first frames.
- App smoke at `http://localhost:1420/` showed `/resources/pets/ian-puppy/sprite.svg` and no browser console/page errors.
- Screenshot: `docs/sdd/specs/0082-puppy-micro-life-upgrade/screenshots/puppy-v2-app-smoke.png`.

## 验收标准映射

- [x] `ian-puppy` Resource Pack 存在，包含 `pet.json`、`animations.json`、`expressions.json`、`sprite.svg`。
- [x] `ian-puppy` 通过 resource loader 校验。
- [x] `ian-puppy` 的主要动画映射到可区分帧或可区分帧序列。
- [x] 新安装默认资源包可显示 `ian-puppy`；已有用户配置不会被强制覆盖。
- [x] idle tick 可在 normal / lively 且非 quiet hours、非输入、非拖拽时触发低打扰微动作。
- [x] quiet mode、quiet hours、bubble input、dragging 状态会阻止自发微动作和高能短链路。
- [x] 跑动或 zoomies 后存在 settle 过渡。
- [x] 拖拽放下后存在 2-4 步短反应链。
- [x] Rust Core 维护 5-10 分钟级别短期日常性格状态，且重启不持久化。
- [x] 短期状态能影响至少两类反馈：刚睡醒短句、刚被摸/拖拽后的微动作强度、跑动 settle。
- [x] React 未根据短期状态自行决定行为，只执行 Rust Core 输出的 `IanAction`。
- [x] 旧 `ian-alpaca` 和 `ian-kitten` 缺少新动画时 fallback 稳定。
- [x] 相关 Rust 和 frontend 测试覆盖资源包、fallback、反应链、阻断条件和短期状态过期。
- [x] 用户验收反馈：puppy 第一版视觉过硬、过丑，已改为圆润短腿视觉。
- [x] 用户验收反馈：run / zoomies / movement 速度过快，已降速并增加确定性节奏变化。

## 剩余风险

- `ian-puppy` 是工程可用 SVG sprite，不是最终美术资产；`rounded-puppy-v2` 已比第一版更软，但后续仍可替换为正式美术资源。
- 当前工作树已有大量非 0082 未提交改动；本次实现没有回滚或清理这些既有改动。
