# Spec: v0.2 Developer Rhythm Acceptance Suite

## 状态

已实现，待验收。

## 问题 / 目标

0041-0049 会把 Developer Rhythm 从 skeleton 推进到可体验能力。0050 目标是建立 v0.2 验收套件，确认 Ian 能低频、可控、隐私安全地理解开发节奏，同时不变成通知器或代码助手。

## 当前产品阶段

v0.2 Acceptance。

## 产品范围

- 汇总授权、workspace、Git、build/test、keyboard、active app、snooze、privacy audit 的验收。
- 建立自动化回归命令。
- 建立 mock event smoke，避免依赖真实项目或终端。
- 明确 v0.2 通过 / 不通过条件。

## 明确不做什么

- 不验收代码分析能力。
- 不验收自动运行测试或 Git 写操作。
- 不验收 Feishu、Pet Visit、插件系统。
- 不要求线上服务。

## 用户体验

通过 0050 后，Ian 应该能在用户显式授权下感知一点开发节奏，并以陪伴方式轻轻回应，不读取敏感内容、不打扰用户。

## 架构约束

- 验收覆盖 Rust Core、Security、Adapters、React settings、Browser smoke。
- 所有 mock event 都必须经过 Security Gate。
- 验收结果记录到 verification。

## 数据 / 协议变化

不新增产品协议。可新增 smoke helper、测试 fixture 或验收文档。

## 隐私与安全边界

验收不能使用真实敏感内容。所有测试 payload 使用低敏 mock 数据。

## 验收标准

- [ ] Developer Rhythm 默认关闭，授权后逐项开启。
- [ ] workspace 未绑定时不产生用户可见开发者反应。
- [ ] Git / build-test mock event 可产生低频 Ian 反应。
- [ ] Keyboard rhythm 和 active app presence 均通过隐私边界测试。
- [ ] snooze / quiet hours / busy category 会降低开发者反应。
- [ ] privacy audit checklist 通过。
- [ ] 验收明确排除代码分析、终端读取、自动命令执行、Feishu、Pet Visit。

## 验证方式

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm run desktop:test`
- `npm run desktop:typecheck`
- `npm run desktop:build`
- Developer Rhythm source scan。
- Browser / mock event smoke。
