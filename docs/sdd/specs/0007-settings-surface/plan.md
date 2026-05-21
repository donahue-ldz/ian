# 实现计划: Settings Surface

## Spec

`docs/sdd/specs/0007-settings-surface/spec.md`

## 状态

已实现，已验证。

## 概要

增加最小设置入口，先覆盖行为模式和少量本地偏好，为后续提醒、BYOM、权限中心预留一致的设置承载方式。

## 步骤

1. 定义最小 settings view model。
2. 增加 Rust command：读取 / 保存设置。
3. 实现轻量 `SettingsPanel`。
4. 接入现有状态同步。
5. 增加测试和 Playwright smoke。

## 预计文件改动

- `apps/desktop/src/renderer/SettingsPanel.tsx`
- `apps/desktop/src/renderer/ianStage.css`
- `apps/desktop/src/lib/tauriBridge.ts`
- `apps/desktop/src-tauri/src/desktop/commands.rs`
- `apps/desktop/src-tauri/src/storage/config.rs`

## 接口与边界

设置变更必须经 Rust command，不能只停留在前端 local state。

## 验证命令

```bash
npm run desktop:test
npm run desktop:typecheck
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml config
```

## 风险

- 设置面过重会稀释桌面生命感；UI 应保持小、可关闭、低打扰。

## 回滚说明

可移除设置入口并保留默认配置，不影响 Ian 基础行为。
