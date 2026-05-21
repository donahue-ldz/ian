# Verification: Kitten Resource Pack

## Status

Implemented and verified on 2026-05-21.

## Planned Commands

```bash
npm --workspace apps/desktop test -- --run src/resources/resourceLoader.test.ts src/renderer/AnimationPlayer.test.ts
cd apps/desktop/src-tauri && cargo test config::tests resource_registry::tests
npm --workspace apps/desktop run dev -- --host 127.0.0.1
```

## Results

| Check | Command / Method | Result | Notes |
| --- | --- | --- | --- |
| RED frontend resource test | `npm --workspace apps/desktop test -- --run src/resources/resourceLoader.test.ts` | Failed as expected | `ian-kitten/pet.json` did not exist before implementation. |
| RED Rust default config test | `cargo test load_state_recovers_corrupt_config_to_default` | Failed as expected | Default `IanState` still returned `ian-alpaca` before implementation. |
| RED Rust resource registry test | `cargo test validates_minimum_resource_pack_contract` | Failed as expected | Empty registry fallback still returned `ian-alpaca` before implementation. |
| Frontend resource/animation tests | `npm --workspace apps/desktop test -- --run src/resources/resourceLoader.test.ts src/renderer/AnimationPlayer.test.ts` | Passed | 2 files, 8 tests passed. |
| Rust default config test | `cargo test load_state_recovers_corrupt_config_to_default` | Passed | 1 test passed. |
| Rust resource registry test | `cargo test validates_minimum_resource_pack_contract` | Passed | 1 test passed. |
| TypeScript typecheck | `npm --workspace apps/desktop run typecheck` | Passed | Initial Node `fs` based test helper failed typecheck; replaced with JSON imports, then passed. |
| Dev server | `npm --workspace apps/desktop run dev -- --host 127.0.0.1 --port 5188` | Passed | Ports `1420` and `5176` were occupied; preview is running at `http://127.0.0.1:5188/`. |
| Browser initial visual smoke test | In-app browser at `http://127.0.0.1:5188/` | Passed | `.ian-sprite-frame` background image is `/resources/pets/ian-kitten/sprite.svg`; initial animation is `idle`. |
| Browser click smoke test | Click Ian button | Passed | Bubble text is `我在这儿。`; animation becomes `happy`. |
| Browser double-click smoke test | Double-click Ian button | Passed | Animation becomes `run`; sprite frame background position moves to run frame. |
| Visual clarity follow-up | In-app browser at `http://127.0.0.1:5188/?kitten=<timestamp>` | Passed | Idle now holds the open-eye kitten frame; screenshot shows brighter gray body, white muzzle/chest, and yellow eyes. |
| Desktop drag regression tests | `npm --workspace apps/desktop test -- --run src/lib/position.test.ts src/renderer/dragGesture.test.ts src/renderer/IanStage.test.tsx src/state/ianActions.test.ts src/resources/resourceLoader.test.ts` | Passed | 5 files, 30 tests passed; includes screen-space physical drag offset and native drag capability contract. |
| TypeScript drag build check | `npm --workspace apps/desktop run typecheck` | Passed | `tsc --noEmit` exited 0 after desktop drag changes. |
| Rust config tests | `cargo test config::tests` | Passed | 4 tests passed. |
| Rust resource registry tests | `cargo test resource_registry::tests` | Passed | 1 test passed. |
| Desktop launch smoke test | `npm --workspace apps/desktop run dev -- --host 127.0.0.1 --port 1420` + `cargo run --no-default-features` | Passed | macOS window list shows `ian_desktop` / `Ian` transparent window at 260x340, and screenshot crop shows the kitten rendered in the desktop window. |
| Automated native mouse drag | Swift `CGEvent` drag over the desktop pet window | Blocked | Current shell reports `AXTrusted=false`; macOS did not deliver synthetic drag events reliably, so this cannot be used as pass/fail evidence without granting Accessibility permission. |

## Remaining Risk

- Current worktree already contains unrelated uncommitted changes outside `0012-kitten-resource-pack`; they were not modified or verified as part of this task.
- Existing local `~/.ian/config.toml` files that explicitly store `ian-alpaca` are not migrated by this task. Browser fallback and new/default state paths use `ian-kitten`.
- Fully automated end-to-end desktop drag verification is blocked by macOS Accessibility permission for synthetic mouse events in this shell. The implementation now uses Tauri native `startDragging()` plus screen-space native window movement fallback, matching the open-source patterns reviewed.
