# 验证记录: Local State Persistence Hardening

## Spec

`docs/sdd/specs/0006-local-state-persistence-hardening/spec.md`

## 状态

已验证。

## 检查项

| 检查项 | 命令 / 方法 | 结果 | 备注 |
| --- | --- | --- | --- |
| Storage tests | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml storage` | 通过 | 3 tests passed。覆盖损坏配置恢复、配置 round-trip、migration 幂等与事件 repository 写入。 |
| Rust check | `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` | 通过 | 无编译错误。 |
| Frontend tests | `npm run desktop:test` | 通过 | 5 files / 14 tests passed。 |
| Position smoke | Tauri dev 拖动后重启 | 部分覆盖 | Rust config round-trip 已覆盖保存/读取；真实桌面拖动重启建议纳入整体验收。 |
| Position startup restore regression | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml desktop::window::tests -- --nocapture` | 通过 | 2 tests passed；已保存 position 优先于右下角默认位置，缺失保存位置时仍回退右下角。 |
| Full regression after restore fix | `npm run desktop:test`; `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`; `npm run desktop:tauri -- dev` | 通过 | 前端 16 tests passed；Rust 28 tests passed；Tauri dev 编译并启动 `target/debug/ian_desktop`。 |

## 验收标准结果

- [x] 拖动 Ian 后重启应用，窗口位置恢复到保存坐标。Rust config round-trip、启动定位选择和 Tauri dev 编译启动已覆盖；真实桌面重启仍建议复跑一次人工 smoke。
- [x] 缺失或损坏的 `config.toml` 会被安全恢复为默认配置。
- [x] SQLite migration 可在空库和已有库上重复执行且不报错。
- [x] 基础互动事件通过 repository 写入，不由 React 直接写存储。
- [x] `npm run desktop:test`、`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml storage` 通过。

## 失败或缺口

真实 Tauri 拖动后退出再启动的位置恢复仍建议在桌面环境中做最终 smoke；自动化已覆盖配置读写链路和启动定位选择链路。

## 后续

整体验收时建议拖动 Ian 到明显位置，关闭并重开应用，确认窗口恢复到上次位置附近。
