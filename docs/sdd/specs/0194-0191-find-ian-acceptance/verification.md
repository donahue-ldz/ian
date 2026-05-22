# 0194 · 验证记录

状态：已执行，真实快捷键人工验收待补。

## 实际结果

| 验证项 | 命令 / 检查 | 结果 | 说明 |
| --- | --- | --- | --- |
| Rust shortcut tests | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml find_ian` | 已由全量 Rust 覆盖 | 全量 153 tests 通过；包括 `find_ian_shortcut_emits_find_actions_without_keyboard_rhythm` 和 protocol low-sensitive 测试。 |
| Frontend tests | `PATH=/opt/homebrew/bin:$PATH npm run desktop:test` | 通过 | 12 files / 91 tests，设置面默认关闭快捷键和冲突提示覆盖。 |
| Typecheck | `PATH=/opt/homebrew/bin:$PATH npm run desktop:typecheck` | 通过 | `tsc --noEmit` exit 0。 |
| Desktop launch | `PATH=/opt/homebrew/bin:$PATH npm run tauri --workspace @ian/desktop -- dev --config '{"build":{"beforeDevCommand":""}}' --no-dev-server-wait` | 通过启动 | Tauri 编译并运行 `target/debug/ian_desktop`，本轮进程已停止。 |

## 剩余风险

- 未在真实桌面按 `Cmd+Shift+I` 做人工验证；需要补验开启、关闭、冲突提示和越界找回。
- 本轮只确认事件链路不包含键盘文本或节奏数据，没有采集任何全局键盘文本。
