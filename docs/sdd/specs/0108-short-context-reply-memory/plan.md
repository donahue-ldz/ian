# 0108 · 实现计划

## 实现步骤

1. 定义短期上下文结构。
2. 补不重复测试。
3. 实现去重选择。
4. 验证不持久化用户正文。

## 预计改动文件

- apps/desktop/src-tauri/src/domain/dialogue/*
- apps/desktop/src-tauri/src/core/creature_state.rs

## 受影响接口或模块

- Rust Core 行为 / 状态 / 存储边界按本 SDD 范围最小调整。
- React 只新增必要渲染、设置或动作执行逻辑。
- Resource Pack 改动必须保持现有 pet.json、animations.json、expressions.json 合同兼容。

## 兼容性说明

- 不破坏现有 ian-adventurer、ian-puppy、ian-kitten、ian-alpaca 的加载。
- 已有用户配置必须保留；除非用户主动触发设置，不强制覆盖本地状态。
- 新增字段应有默认值或迁移策略。

## 验证命令和桌面验收

- 目标测试：按实际改动运行最小相关测试。
- 前端检查：PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:test -- <target>。
- 类型检查：PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:typecheck。
- Rust 检查：cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml <target>。
- 桌面验收：PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:tauri -- dev，在真实 Tauri 桌面壳中执行本 SDD 的用户路径。

## 风险和回滚

- 风险：桌面透明窗口中的实际观感和浏览器 / SVG 预览不一致。
  - 应对：真实 Tauri 桌面验收必须记录到 verification.md。
- 风险：局部体验增强可能让 Ian 显得打扰。
  - 应对：保留冷却、quiet mode、设置开关或回退默认。
- 回滚：移除本 SDD 新增入口或策略，恢复上一版 resource pack / 行为策略 / 设置默认值。
