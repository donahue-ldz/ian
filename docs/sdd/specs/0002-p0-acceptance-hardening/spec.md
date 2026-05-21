# Spec: P0 验收收口与集成

## 状态

草稿，等待用户确认。

## 背景

0001 `P0 本地数字生命验证版` 已在独立 worktree 中实现到可验证状态，但检查发现它尚未形成可合并的完成态：

- 实现位于 `sdd/0001-p0-local-creature-proof` worktree，尚未提交或合并到 `main`。
- 自动验证通过：frontend typecheck、frontend build、unit tests、Rust check、Rust format。
- 0001 verification 中存在验收口径冲突：验收标准勾选了“右下角窗口”，但缺口记录说明当前为了人工验收临时使用 360x360 居中窗口。
- 位置持久化端到端验证尚未完成。
- `src/protocol/generated.ts` 仍是明确标注的 generated placeholder，尚未接通 Rust -> TypeScript 类型生成。

0002 的目标不是扩展新产品功能，而是让 P0 进入可以合并、可继续迭代的稳定基线。

## 目标

完成 P0 验收收口：

- 将 0001 的实现整理成可提交、可合并状态。
- 解决或明确记录右下角窗口与居中验收之间的冲突。
- 完成位置持久化验证或将其降级为明确未完成项。
- 明确 `generated.ts` 的短期策略：接通生成，或保留 placeholder 并记录后续任务。
- 更新 0001 verification，使验收状态真实反映实现情况。
- 合并 0001 实现前，确保不引入未来阶段用户可见功能。

## 当前阶段

```txt
P0 / MVP + v0.1 Architecture Baseline
```

## 产品范围

0002 只处理 P0 收口问题：

- 右下角默认位置 / 验收口径修正。
- 基础位置或配置持久化验证。
- 0001 实现分支清理、提交和合并准备。
- SDD verification 与实际状态对齐。
- 架构 skeleton 的完整性核对。

## 非目标

0002 不做新产品能力：

- 不做 Git / build / test / CI 集成。
- 不做全局键盘监听。
- 不做完整 Mood System。
- 不做完整 Bond System。
- 不做长期记忆。
- 不做主动提醒系统。
- 不做 Feishu。
- 不做 Pet Visit。
- 不做 plugin system。
- 不做 window-aware behavior。
- 不做 BYOM UI。
- 不替换正式美术资源，除非只是修复 P0 placeholder 的加载问题。

## 用户体验

0002 完成后，P0 应有清晰、可复现的用户验收状态：

- app 可以启动。
- Ian 窗口位置与 spec 一致，或者 spec 明确接受当前临时形态。
- 点击气泡和双击乱跑仍可工作。
- 如果位置持久化被声明完成，则有明确验证记录。
- 如果位置持久化未完成，则不得在 verification 中勾选完成。

## 架构约束

- 不得把 0002 变成新功能迭代。
- Rust Core 仍是行为大脑。
- React 仍只渲染 `IanAction`。
- 事件流仍保持 `IanEvent` -> Rust Core -> `IanAction`。
- 未来能力只能以 skeleton 形式存在，不能成为 P0 用户可见功能。
- 任何验收降级或范围调整必须记录到 `decisions.md`。

## 数据与协议变化

预期不新增 P0 用户可见协议能力。

允许的技术调整：

- 修正 `IanState.position` 与窗口位置恢复之间的实际连接。
- 修正 config 持久化路径或读写逻辑。
- 接通 Rust -> TypeScript 类型生成，或将 placeholder 策略记录为后续任务。

## 隐私与安全

0002 不新增敏感数据读取。

仍然禁止：

- 全局键盘输入
- clipboard
- code content
- private chat content
- screen OCR
- terminal output

## 验收标准

- [ ] 0001 实现所在 worktree 的状态被明确记录：已提交、已合并，或仍为待合并。
- [ ] `node_modules`、`dist`、`target` 等生成目录不会被提交。
- [ ] frontend typecheck 通过。
- [ ] frontend build 通过。
- [ ] frontend unit tests 通过。
- [ ] Rust check 通过。
- [ ] Rust format check 通过。
- [ ] 0001 verification 中的“右下角窗口”与实际窗口行为不再矛盾。
- [ ] 位置持久化若被勾选完成，则 verification 中有实际验证记录；否则保持未勾选并说明原因。
- [ ] `generated.ts` 的状态被明确记录：已由 Rust 生成，或作为 P0 placeholder 并有后续任务。
- [ ] 0002 不引入任何 P0 非目标功能。

## 验证方式

应至少运行：

```bash
npm run desktop:typecheck
npm run desktop:build
npm run desktop:test
cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml
cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml --check
git status --short --ignored
```

必要时补充人工检查：

- app 启动位置。
- 点击气泡。
- 双击乱跑。
- 拖拽后重启的位置恢复。

## 开放问题

- 0002 是否应该直接修复 0001 的右下角窗口位置，还是把“居中窗口”正式记录为 P0 验收形态？
- 位置持久化是否必须在 0002 中完成，还是允许进入后续任务？
- `ts-rs` 生成是否必须在 0002 中接通，还是允许保留 placeholder 到后续架构任务？

