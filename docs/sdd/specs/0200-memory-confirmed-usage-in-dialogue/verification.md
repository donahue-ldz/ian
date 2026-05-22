# 0200 · 验证记录

状态：已执行，待用户验收。

## 实际结果

| 验证项 | 命令 / 检查 | 结果 | 说明 |
| --- | --- | --- | --- |
| RED | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml memory -- --nocapture` | 失败符合预期 | `DialogueEngine::reply_to` 不支持 confirmed memory tags。 |
| GREEN | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml memory -- --nocapture` | 通过 | `dialogue_uses_only_confirmed_memory_tags_as_small_context` 通过。 |
| Full Rust tests | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` | 通过 | 153 tests。 |
| Frontend tests | `PATH=/opt/homebrew/bin:$PATH npm run desktop:test` | 通过 | 12 files / 91 tests。 |

## 剩余风险

- Demo Dialogue 目前只轻量消费 `pref:quiet`；更多 confirmed tags 的表达需要后续单独调优。
