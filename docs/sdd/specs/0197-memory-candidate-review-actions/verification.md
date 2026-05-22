# 0197 · 验证记录

状态：已执行，待用户验收。

## 实际结果

| 验证项 | 命令 / 检查 | 结果 | 说明 |
| --- | --- | --- | --- |
| RED | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml memory -- --nocapture` | 失败符合预期 | 缺少 `candidates` / `clear_candidates` 和 review 相关方法。 |
| GREEN | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml memory -- --nocapture` | 通过 | 7 个 memory/dialogue 相关 tests 通过。 |
| Full Rust tests | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` | 通过 | 153 tests。 |
| Frontend tests | `PATH=/opt/homebrew/bin:$PATH npm run desktop:test` | 通过 | 12 files / 91 tests。 |

## 剩余风险

- 未在真实设置面点击确认、删除、清空按钮；Tauri command 和前端渲染路径已覆盖，人工交互待补。
