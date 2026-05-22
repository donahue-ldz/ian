# 0193 · 验证记录

状态：已执行，桌面人工三屏拖动待补验。

## 实际结果

| 验证项 | 命令 / 检查 | 结果 | 说明 |
| --- | --- | --- | --- |
| Frontend regression | `PATH=/opt/homebrew/bin:$PATH npm run desktop:test` | 通过 | 12 files / 91 tests，覆盖 dragGesture / IanStage 回归。 |
| Typecheck | `PATH=/opt/homebrew/bin:$PATH npm run desktop:typecheck` | 通过 | `tsc --noEmit` exit 0。 |
| Rust tests | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` | 通过 | 153 tests。 |
| Desktop launch | `PATH=/opt/homebrew/bin:$PATH npm run tauri --workspace @ian/desktop -- dev --config '{"build":{"beforeDevCommand":""}}' --no-dev-server-wait` | 通过启动 | 复用已有 `localhost:1420` dev server；Tauri 编译完成并运行 `target/debug/ian_desktop`。本轮启动的 PID 已停止。 |

## 剩余风险

- 首次 `npm run desktop:tauri -- dev` 因本机已有 `1420` dev server 失败；随后用 Tauri config override 跳过 beforeDevCommand 成功启动。
- 未执行真实三屏人工拖动观察；需要按 `docs/sdd/desktop-smoke-checklist.md` 补验拖动不跳动、不回弹、释放位置保存和重启恢复。
