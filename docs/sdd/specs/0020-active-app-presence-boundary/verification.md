# 验证记录: Active App Presence Boundary

## Spec

`docs/sdd/specs/0020-active-app-presence-boundary/spec.md`

## 状态

已验证，待用户验收。

## 检查项

| 检查项 | 命令 / 方法 | 结果 | 备注 |
| --- | --- | --- | --- |
| Adapter tests | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml adapters` | 通过 | active app adapter skeleton 默认关闭。 |
| Security tests | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml security` | 通过 | 权限和 payload 脱敏已覆盖。 |
| Behavior tests | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior` | 通过 | app category 降低打扰边界已覆盖。 |
| Source scan | `rg -n "window title|document\\.title|url|ocr|screen" apps/desktop/src-tauri/src apps/desktop/src` | 通过 | 无匹配。 |

## 验收标准结果

- [x] Active app presence adapter skeleton 默认关闭。
- [x] 未授权事件被 Security Gate 拒绝。
- [x] payload 不包含窗口标题、URL、文件名或屏幕文字。
- [x] BehaviorPolicy 可基于 app category 降低提醒打扰。
- [x] 测试覆盖默认关闭、payload 脱敏、policy 使用 category。

## 失败或缺口

本批只做 category boundary，不做真实窗口标题、URL 或屏幕内容读取。

## 后续

实现后补充实际验证结果。
