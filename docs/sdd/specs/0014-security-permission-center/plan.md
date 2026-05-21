# 实现计划: Security Permission Center

## Spec

`docs/sdd/specs/0014-security-permission-center/spec.md`

## 状态

已实现，待验收。

## 概要

在扩展 v0.2 能力前，把权限和敏感源边界做成可检查、可展示、默认安全的结构。

## 步骤

1. 增加 Security Gate 单元测试。
2. 定义 sensitivity/source/permission model。
3. 接入 sanitizer 和 payload limit。
4. 在设置面展示当前启用能力。
5. 确认默认配置不启用高敏感能力。

## 预计文件改动

- `apps/desktop/src-tauri/src/security/*`
- `apps/desktop/src-tauri/src/adapters/*`
- `apps/desktop/src-tauri/src/storage/config.rs`
- `apps/desktop/src/renderer/SettingsPanel.tsx`

## 接口与边界

权限判断在 Rust，不在 React；React 只展示和提交用户选择。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml security
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml config
npm run desktop:test
```

## 风险

- 权限面过复杂会增加心智负担；0014 只展示当前能力，不展示未来功能营销入口。

## 回滚说明

保留 Security Gate 默认 allowlist，移除设置展示。
