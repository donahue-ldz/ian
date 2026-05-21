# 实现计划: Creature Settings Depth

## 对应规格

`docs/sdd/specs/0038-creature-settings-depth/spec.md`

## 实现步骤

1. 为扩展设置 schema 写 config 失败测试。
2. 扩展 Rust config 和默认值。
3. 将设置接入 BehaviorPolicy / Scheduler。
4. 更新 SettingsPanel 和 settings model。
5. 补充前端测试和 browser smoke。

## 预计改动文件

- `apps/desktop/src-tauri/src/storage/config.rs`
- `apps/desktop/src-tauri/src/domain/behavior/behavior_policy.rs`
- `apps/desktop/src-tauri/src/core/scheduler.rs`
- `apps/desktop/src/renderer/SettingsPanel.tsx`
- `apps/desktop/src/renderer/settingsModel.ts`
- `docs/sdd/specs/0038-creature-settings-depth/*`

## 接口 / 兼容性

旧配置缺字段时使用默认值。设置项应保持少量、清晰，不引入规则表达式。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml storage
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
npm run desktop:test
npm run desktop:typecheck
```

## 风险和回滚

设置过多会破坏轻量体验。回滚方式是只保留 movement intensity 和 quiet hours 两组设置。
