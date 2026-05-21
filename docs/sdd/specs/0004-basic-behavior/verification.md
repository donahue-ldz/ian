# 验证记录: Basic Behavior

## 状态

已验证。

## 检查项

| 检查项 | 命令 / 方法 | 结果 | 备注 |
| --- | --- | --- | --- |
| TDD RED | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior` | 失败符合预期 | 缺少 `IanEvent::MouseNear`。 |
| Rust behavior tests | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior` | 通过 | 4 个 behavior/protocol 相关过滤测试通过。 |
| Full Rust tests | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` | 通过 | 11 个 Rust 测试通过。 |
| Frontend test | `npm run desktop:test` | 通过 | 3 个测试文件，9 个测试通过。 |
| Frontend typecheck | `npm run desktop:typecheck` | 通过 | `tsc --noEmit` 通过。 |
| Playwright | `http://localhost:1420/` | 通过 | pointer near 后 `data-animation="happy"`；双击 run 后约 2.1s 回到 idle；console 无 error。 |

## 验收标准结果

- [x] Rust protocol 包含 `mouse.near`。
- [x] 前端 pointer enter 发送 `mouse.near`。
- [x] Rust Core 对 `mouse.near` 返回 `animation.play happy` 或等价轻反应。
- [x] `time.tick` 可返回 idle/walk/sleep 中的基础动作，且决策可测试。
- [x] React 不直接决定行为。
- [x] Playwright 可验证 pointer 进入后动画状态变化。
