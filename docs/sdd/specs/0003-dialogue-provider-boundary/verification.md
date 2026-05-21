# 验证记录: Dialogue Provider Boundary

## 状态

已验证。

## 检查项

| 检查项 | 命令 / 方法 | 结果 | 备注 |
| --- | --- | --- | --- |
| TDD RED | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml dialogue` | 失败符合预期 | 缺少 `DialogueProvider`、`DialogueContext`、`DialogueSource`。 |
| Rust dialogue tests | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml dialogue` | 通过 | 1 个 dialogue 测试通过。 |
| Full Rust tests | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` | 通过 | 11 个 Rust 测试通过。 |
| Rust check | `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` | 通过 | Tauri Rust app 可编译。 |
| Rust fmt | `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml --check` | 通过 | 格式检查通过。 |

## 验收标准结果

- [x] Rust Core 存在 `DialogueProvider` trait，Demo Provider 实现该 trait。
- [x] `DialogueEngine` 通过 provider trait 生成回复，而不是直接调用具体 demo 方法。
- [x] `DialogueContext` 至少包含当前 behavior、animation 和 source。
- [x] Demo 回复经过 `DialoguePolicy` 裁剪，最大长度可测试。
- [x] `DialogueUserMessage` 仍返回 `speech.show` 和 `animation.play`。
- [x] 前端测试、Rust 检查通过。
