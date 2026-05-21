# 实现计划: Developer Rhythm Onboarding

## 对应规格

`docs/sdd/specs/0041-developer-rhythm-onboarding/spec.md`

## 实现步骤

1. 为 capability 默认关闭和持久化写 Rust 回归测试。
2. 为 SettingsPanel Developer Rhythm 分组写前端测试。
3. 更新设置面文案，明确每项能力的数据范围。
4. 确认 Security Gate 使用同一授权状态。
5. 运行 browser smoke 并记录结果。

## 预计改动文件

- `apps/desktop/src-tauri/src/storage/config.rs`
- `apps/desktop/src-tauri/src/security/permission.rs`
- `apps/desktop/src/renderer/SettingsPanel.tsx`
- `apps/desktop/src/renderer/settingsModel.ts`
- `docs/sdd/specs/0041-developer-rhythm-onboarding/*`

## 接口 / 兼容性

复用现有 capability 字段。旧配置缺字段时默认关闭。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml storage
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml security
npm run desktop:test
npm run desktop:typecheck
```

## 风险和回滚

设置面可能过重。回滚方式是保留逐项开关，把说明压缩为每项一行。
