# 实现计划: Project Workspace Binding

## 对应规格

`docs/sdd/specs/0042-project-workspace-binding/spec.md`

## 实现步骤

1. 为 workspace config round-trip 写失败测试。
2. 增加 workspace binding model 和默认未绑定状态。
3. 让 Git / build-test adapter 检查 workspace boundary。
4. 在设置面展示绑定状态和解绑入口。
5. 做 source scan，确认没有源码内容读取。

## 预计改动文件

- `apps/desktop/src-tauri/src/storage/config.rs`
- `apps/desktop/src-tauri/src/adapters/git_metadata_adapter.rs`
- `apps/desktop/src-tauri/src/adapters/build_test_adapter.rs`
- `apps/desktop/src/renderer/SettingsPanel.tsx`
- `docs/sdd/specs/0042-project-workspace-binding/*`

## 接口 / 兼容性

旧配置无 workspace 时保持未绑定。已有手动 mock event 不受影响，但不能产生用户可见 workspace 反应。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml storage
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml adapters
npm run desktop:test
```

## 风险和回滚

路径展示可能暴露用户目录名。回滚方式是只显示项目名和是否绑定，不显示完整路径。
