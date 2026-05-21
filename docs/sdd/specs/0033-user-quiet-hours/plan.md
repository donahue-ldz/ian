# 实现计划: User Quiet Hours

## 对应规格

`docs/sdd/specs/0033-user-quiet-hours/spec.md`

## 实现步骤

1. 为 quiet hours config round-trip 和跨午夜判断写失败测试。
2. 扩展配置模型和默认值。
3. 在 BehaviorPolicy / Scheduler 中应用 quiet hours。
4. 在设置面增加最小 quiet hours 控件。
5. 补充前端测试和 browser smoke。

## 预计改动文件

- `apps/desktop/src-tauri/src/storage/config.rs`
- `apps/desktop/src-tauri/src/domain/behavior/behavior_policy.rs`
- `apps/desktop/src-tauri/src/core/scheduler.rs`
- `apps/desktop/src/renderer/SettingsPanel.tsx`
- `apps/desktop/src/renderer/settingsModel.ts`
- `docs/sdd/specs/0033-user-quiet-hours/*`

## 接口 / 兼容性

旧配置缺少 quiet hours 时应使用 disabled 默认值。配置损坏继续回退默认。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml storage
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
npm run desktop:test
npm run desktop:typecheck
```

## 风险和回滚

设置 UI 可能变复杂。回滚方式是先只提供开关和固定时间段，后续再扩展时间选择。
