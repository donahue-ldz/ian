# 实现计划: Developer Snooze And Busy Policy

## 对应规格

`docs/sdd/specs/0048-developer-snooze-and-busy-policy/spec.md`

## 实现步骤

1. 为 snooze config 和到期判断写失败测试。
2. 在 DeveloperRhythmPolicy 中接入 snooze / quiet hours / busy category。
3. 在设置面增加 snooze 控件。
4. 确认 click / drag / dialogue 不受 snooze 影响。
5. 做 browser smoke。

## 预计改动文件

- `apps/desktop/src-tauri/src/storage/config.rs`
- `apps/desktop/src-tauri/src/domain/behavior/developer_rhythm_policy.rs`
- `apps/desktop/src-tauri/src/app/runtime.rs`
- `apps/desktop/src/renderer/SettingsPanel.tsx`
- `docs/sdd/specs/0048-developer-snooze-and-busy-policy/*`

## 接口 / 兼容性

旧配置无 snooze 时默认关闭。snooze 到期后自动恢复。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml storage
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
npm run desktop:test
npm run desktop:typecheck
```

## 风险和回滚

snooze UI 可能增加复杂度。回滚方式是先支持“暂停 30 分钟”一个选项。
