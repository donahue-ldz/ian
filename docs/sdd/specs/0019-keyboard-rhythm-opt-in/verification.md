# 验证记录: Keyboard Rhythm Opt In

## Spec

`docs/sdd/specs/0019-keyboard-rhythm-opt-in/spec.md`

## 状态

已验证，待用户验收。

## 检查项

| 检查项 | 命令 / 方法 | 结果 | 备注 |
| --- | --- | --- | --- |
| Adapter tests | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml adapters` | 通过 | opt-out 后停止采集已覆盖。 |
| Security tests | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml security` | 通过 | 默认拒绝和 payload 脱敏已覆盖。 |
| Source scan | `rg -n "keypress|keydown|shortcut|Command::new" apps/desktop/src-tauri/src apps/desktop/src` | 通过 | 无真实键盘监听路径。 |

## 验收标准结果

- [x] KeyboardRhythmAdapter 默认关闭。
- [x] 未授权时 Security Gate 拒绝 rhythm event。
- [x] 授权后 payload 不包含 key、text、shortcut。
- [x] 关闭授权后停止采集。
- [x] 测试覆盖默认关闭、授权开启、payload 脱敏。

## 失败或缺口

本批不实现真实全局键盘监听，只保留 opt-in 摘要边界。

## 后续

实现后补充实际验证结果。
