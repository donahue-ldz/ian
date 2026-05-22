# 0125 · 实现计划

## 实现步骤

1. 盘点已有实现和相关 SDD，确认不重复前序范围。
2. 先补最小 RED 测试或验收清单，锁定本 SDD 的客观结果。
3. 按 Rust Core / React / Storage / Security 边界做最小实现或文档化骨架。
4. 运行目标测试、类型检查或 Rust 检查。
5. 如涉及桌面可见能力，启动真实 Tauri 桌面壳验收并更新 verification.md。

## 预计改动文件

- apps/desktop/src-tauri/src/domain/adapter/*
- apps/desktop/src-tauri/src/security/*
- apps/desktop/src/renderer/SettingsPanel.tsx

## 受影响接口或模块

- Rust Core domain / policy / storage / security 边界按本 SDD 范围最小调整。
- React 只新增必要设置、渲染或执行逻辑。
- 如涉及 adapter，必须声明敏感度、默认状态和权限要求。

## 兼容性说明

- 不破坏 P0 已有桌面生命感、点击、拖动、气泡、资源包切换和位置恢复。
- 已有用户配置必须保留；新增配置必须有默认值和失败回退。
- 高敏能力即使有 skeleton，也不能默认启用。

## 验证命令和桌面验收

- 前端目标测试：PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:test -- <target>。
- 类型检查：PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:typecheck。
- Rust 目标测试：cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml <target>。
- 桌面验收：PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:tauri -- dev。

## 风险和回滚

- 风险：未来能力 skeleton 被误认为已可用产品能力。
  - 应对：默认关闭，设置文案明确标注授权和阶段。
- 风险：体验增强增加打扰感。
  - 应对：保留冷却、quiet mode、关闭开关和回退默认。
- 回滚：删除本 SDD 新增入口或恢复默认关闭状态；保留无害 schema 时必须记录兼容原因。
