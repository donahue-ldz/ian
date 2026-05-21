# 实现计划: P0 验收收口与集成

## Spec

`docs/sdd/specs/0002-p0-acceptance-hardening/spec.md`

## 状态

草稿，等待用户确认。

## 概要

0002 不是新功能开发，而是对 0001 的实现进行验收收口、记录修正和合并准备。

核心工作：

- 核对 0001 worktree 的实际实现。
- 运行验证命令。
- 修正 verification 与实际行为之间的矛盾。
- 判断是否需要小范围修复窗口位置、位置持久化或协议生成。
- 清理生成文件提交边界。
- 准备将 0001 合并回 `main`。

## 验收标准映射

| 实现步骤 | 对应验收标准 |
| --- | --- |
| 1. 记录 worktree 状态 | worktree 状态明确记录 |
| 2. 清理提交边界 | 生成目录不会被提交 |
| 3. 运行自动验证 | typecheck/build/test/Rust check/Rust fmt 通过 |
| 4. 对齐窗口位置验收 | 右下角窗口与实际行为不再矛盾 |
| 5. 对齐位置持久化验收 | 完成则有记录，未完成则保持未勾选 |
| 6. 对齐协议生成策略 | generated.ts 状态明确 |
| 7. 更新 SDD 记录 | verification / decisions 真实反映状态 |
| 8. 准备合并 | 0002 不引入非目标功能 |

## 步骤

1. 检查 0001 worktree。
   - 查看 `git status --short --ignored`。
   - 确认实现文件、生成文件、依赖目录的状态。
   - 确认是否已有未提交用户改动。

2. 运行验证命令。
   - `npm run desktop:typecheck`
   - `npm run desktop:build`
   - `npm run desktop:test`
   - `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
   - `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml --check`

3. 核对 0001 verification。
   - 将自动验证结果与 `verification.md` 对齐。
   - 找出已勾选但证据不足的验收项。
   - 找出已完成但未记录的验收项。

4. 决策窗口默认位置。
   - 如果坚持 P0 spec 的右下角要求，则修复窗口初始位置。
   - 如果接受居中窗口作为 P0 临时验收形态，则更新 spec / decisions / verification 说明。

5. 决策位置持久化。
   - 如果实现已具备端到端能力，执行或记录人工验证。
   - 如果未具备端到端能力，保持验收未完成，并记录后续任务。

6. 决策协议生成。
   - 优先评估是否能低成本接通 `ts-rs` 生成。
   - 如果接通成本超过 0002 范围，保留 placeholder，并在 decisions / verification 中明确记录。

7. 清理提交内容。
   - 确保 `node_modules`、`dist`、`target` 不进入提交。
   - 确认 `.gitignore` 覆盖生成目录。
   - 确认 `package-lock.json`、`Cargo.lock` 是否应提交。

8. 更新 SDD 文档。
   - 更新 0001 verification。
   - 更新 0001 decisions，如有验收范围调整。
   - 更新 0002 verification，记录本次检查与结果。

9. 准备集成。
   - 若验证通过且提交边界清晰，提交 0001 worktree 实现。
   - 由用户确认后，再决定是否 merge 到 `main`。

## 预计文件改动

可能改动：

- `docs/sdd/specs/0001-p0-local-creature-proof/verification.md`
- `docs/sdd/specs/0001-p0-local-creature-proof/decisions.md`
- `docs/sdd/specs/0002-p0-acceptance-hardening/verification.md`
- `docs/sdd/specs/0002-p0-acceptance-hardening/decisions.md`
- `apps/desktop/src-tauri/src/desktop/window.rs`
- `apps/desktop/src/lib/position.ts`
- `apps/desktop/src/state/useIanActions.ts`
- `.gitignore`

只有在需要小范围修复验收缺口时，才修改 app 代码。

## 接口与边界

- 不新增用户可见功能。
- 不新增高敏权限。
- 不改变 `IanEvent` / `IanAction` / `IanState` 的核心方向。
- 如需修复位置恢复，应继续通过 Rust Core / Tauri command / frontend action 边界完成，不把长期行为逻辑塞进 React。

## 验证命令

```bash
npm run desktop:typecheck
npm run desktop:build
npm run desktop:test
cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml
cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml --check
git status --short --ignored
```

## 风险

- 0001 worktree 目前存在大量未提交文件，直接合并前需要明确提交边界。
- 如果居中窗口只是临时验收形态，继续勾选“右下角窗口”会污染验收记录。
- 位置持久化可能需要桌面人工操作验证，自动化成本可能超过 0002 范围。
- 接通 `ts-rs` 生成可能引入额外 build 脚本复杂度。

## 回滚说明

- SDD 文档可单独回滚。
- 若只修改 verification / decisions，不影响产品代码。
- 若修复窗口位置或持久化，可通过回退对应文件恢复 0001 实现状态。

