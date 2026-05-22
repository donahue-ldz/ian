# 0218 · 实现计划

## 实现步骤

1. 开始前收口当前工作树，确认是否存在同一范围的未验收改动。
2. 阅读本 SDD 的 spec、相关历史 SDD、当前代码和现有测试，确认不重复实现已完成能力。
3. 写最小 RED 测试、验收清单或文档断言，覆盖本 SDD 的关键验收标准。
4. 按 Rust Core / React / Storage / Security / Resource Pack 的职责边界做最小实现或文档更新。
5. 如发现范围变化，先更新 decisions.md，再继续实现。
6. 运行目标测试、类型检查和必要构建。
7. 涉及桌面可见能力时，启动真实 Tauri 桌面壳验收。
8. 更新 verification.md，记录实际结果和剩余风险。

## 预计改动文件

- SettingsPanel.tsx
- permission_registry.rs
- docs/codex/privacy

## 受影响接口或模块

- Rust Core / BehaviorPolicy：只在本 SDD 明确需要时新增决策逻辑。
- React / Renderer：只负责展示、设置入口、用户操作和 IanAction 执行。
- Storage / Config：新增字段必须有默认值、迁移和回滚说明。
- Security / Permission：新增外部输入必须声明敏感度、默认状态和拒绝路径。
- Docs / SDD：文档状态必须和实际实现、验证结果一致。

## 兼容性说明

- 不破坏 P0 已有桌面生命感、点击、拖动、气泡、资源包切换和位置恢复。
- 已有用户配置必须保留；新增配置必须可缺省读取。
- future skeleton 只能默认关闭，不能变成未授权用户可见能力。

## 验证命令和桌面验收

- 前端目标测试：

```bash
PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:test
```

- 类型检查：

```bash
PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:typecheck
```

- Rust 目标测试：

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
```

- 桌面验收，如本 SDD 涉及桌面可见行为：

```bash
PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:tauri -- dev
```

## 风险和回滚

- 风险：误把 skeleton 当作完整产品能力。
  - 应对：默认关闭、文案说明、verification 记录真实状态。
- 风险：实现越过 Rust Core 或 Security 边界。
  - 应对：新增测试锁定 IanEvent / IanAction 链路和拒绝路径。
- 风险：桌面行为只在浏览器通过。
  - 应对：真实 Tauri 桌面壳验收。
- 回滚：恢复本 SDD 新增入口、配置或策略到默认关闭；若保留无害 schema，必须记录兼容原因。
