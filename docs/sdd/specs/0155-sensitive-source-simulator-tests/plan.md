# 0155 · 实现计划

## 实现步骤

1. 盘点已有 SDD、代码和架构边界，确认不重复前序范围。
2. 写出最小 RED 测试、验收清单或安全拒绝用例。
3. 按 Rust Core / Adapter / Storage / Security / React 职责边界做最小实现或 skeleton。
4. 更新 decisions.md，记录任何范围变化、默认关闭策略或迁移取舍。
5. 运行目标测试、类型检查或 Rust 检查。
6. 如涉及桌面可见能力，启动真实 Tauri 桌面壳验收并更新 verification.md。

## 预计改动文件

- apps/desktop/src-tauri/src/domain/adapter/*
- apps/desktop/src-tauri/src/domain/behavior/*
- apps/desktop/src-tauri/src/security/*
- apps/desktop/src/renderer/SettingsPanel.tsx

## 受影响接口或模块

- Rust Core domain / policy / storage / security 边界按本 SDD 范围最小调整。
- React 只新增必要设置、渲染或动作执行逻辑。
- 如涉及 adapter，必须声明敏感度、默认状态、payload 限制和权限要求。
- 如涉及 future skeleton，必须默认关闭并有测试锁定。

## 兼容性说明

- 不破坏 P0 已有桌面生命感、点击、拖动、气泡、资源包切换和位置恢复。
- 已有用户配置必须保留；新增配置必须有默认值、迁移或失败回退。
- 高敏能力即使有 skeleton，也不能默认启用。

## 验证命令和桌面验收

- 前端目标测试：PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:test -- <target>。
- 类型检查：PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:typecheck。
- Rust 目标测试：cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml <target>。
- 桌面验收：PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:tauri -- dev。

## 风险和回滚

- 风险：未来能力 skeleton 被误认为已可用产品能力。
  - 应对：默认关闭，设置文案明确标注授权和阶段。
- 风险：Developer Rhythm 或 Social Presence 让 Ian 变成通知器。
  - 应对：摘要事件、冷却、短句、用户控制和关闭开关。
- 风险：隐私边界被实现细节绕过。
  - 应对：Security Gate 默认拒绝、payload budget、脱敏测试和审计记录。
- 回滚：删除本 SDD 新增入口或恢复默认关闭状态；保留无害 schema 时必须记录兼容原因。
