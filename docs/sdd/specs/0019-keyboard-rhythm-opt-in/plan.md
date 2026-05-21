# 实现计划: Keyboard Rhythm Opt In

## Spec

`docs/sdd/specs/0019-keyboard-rhythm-opt-in/spec.md`

## 状态

已实现，待验收。

## 概要

引入显式授权的键盘节奏摘要能力，只采集强度和计数，不采集内容。

## 步骤

1. 定义权限项和 rhythm summary event。
2. 编写默认关闭和未授权拒绝测试。
3. 实现 no-content rhythm collector。
4. 接入设置开关。
5. 做 payload scan 和 opt-out 验证。

## 预计文件改动

- `apps/desktop/src-tauri/src/adapters/*`
- `apps/desktop/src-tauri/src/security/*`
- `apps/desktop/src-tauri/src/protocol/event.rs`
- `apps/desktop/src/renderer/SettingsPanel.tsx`

## 接口与边界

键盘节奏只允许摘要，不允许 key/text 进入 Runtime 或 storage。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml adapters
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml security
rg -n "key|text|shortcut|keydown|keypress" apps/desktop/src-tauri/src
```

## 风险

- 这是高敏能力；必须能清楚关闭，默认不采集。

## 回滚说明

移除 adapter 注册或保持 permission false。
