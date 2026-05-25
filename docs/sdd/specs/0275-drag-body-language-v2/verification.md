# 0275 · 验证记录

## 状态

通过，连续真实拖动次数需人工补验。

## 自动化验证

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：通过，181 个 Rust 测试全部通过。
- `PATH=/Users/bytedance/Git/ian/node_modules/.bin:$PATH vitest run src/renderer/IanStage.test.tsx`：通过，16 个测试全部通过。
- `PATH=/Users/bytedance/Git/ian/node_modules/.bin:$PATH vitest run`：通过，16 个测试文件 / 121 个测试全部通过。
- `PATH=/Users/bytedance/Git/ian/node_modules/.bin:$PATH tsc --noEmit`：通过。

## 覆盖结果

- `drag_start_sets_carry_state_and_blocks_other_moments_while_dragging` 覆盖 drag start 的 carry state，以及拖动中 chase / idle Moment 被抑制。
- `IanStage` 测试覆盖 `data-drag-phase="resting"` 和 pickup / carried / dropping CSS。
- 既有 position 测试继续覆盖桌面拖动位置持久化计算。

## 桌面验证

- 真实 Tauri 桌面壳已启动，截图 `/tmp/ian-0273-0276-desktop.png` 确认 Ian 可见且仍在桌面窗口内。
- 当前环境 `System Events` 辅助功能权限为 `false`，无法自动执行“连续拖动至少 5 次”的桌面手感验收。

## 剩余风险

- 需要人工连续拖动 Ian 至少 5 次，确认 pickup / carried / dropping 姿态不影响命中和最终落点。
