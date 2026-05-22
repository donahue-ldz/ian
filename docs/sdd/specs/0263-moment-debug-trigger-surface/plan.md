# 0263 · 实施计划

## 步骤

1. 查找现有设置面板、诊断面板或 Tauri command 入口。
2. 设计最小诊断触发事件，保持 Rust Core 为行为大脑。
3. 为每个 Moment 增加确定性触发路径。
4. 在前端提供轻量入口，避免影响普通用户主界面。
5. 补充测试和真实桌面触发验收。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/moment_orchestrator.rs`
- `apps/desktop/src-tauri/src/desktop/commands.rs`
- `apps/desktop/src/renderer/SettingsPanel.tsx`
- `apps/desktop/src/state/ianActions.ts`
- 相关测试文件
- `docs/sdd/specs/0263-moment-debug-trigger-surface/verification.md`

## 受影响接口或模块

- Rust Core Moment Orchestrator
- Tauri command / desktop bridge
- React settings 或 diagnostics surface

## 兼容性说明

新增诊断入口必须默认低风险，不破坏现有 Moment 随机和冷却策略。

## 验证命令或手动检查

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm run desktop:test`
- `npm run desktop:typecheck`
- 真实 Tauri 桌面逐项触发 Moment。

## 风险和回滚

- 风险：诊断入口被误当成正式功能。
- 缓解：命名和 UI 明确为诊断；必要时只在开发模式显示。
- 回滚：删除入口和诊断事件，不影响正式 Moment。
