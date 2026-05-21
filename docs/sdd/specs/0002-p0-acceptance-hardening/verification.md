# 验证记录: P0 验收收口与集成

## Spec

`docs/sdd/specs/0002-p0-acceptance-hardening/spec.md`

## 状态

草稿。已完成启动前检查，尚未开始 0002 实现。

## 验证摘要

已对 0001 worktree 做初步检查：

- 0001 实现位于独立 worktree：`/Users/bytedance/.config/superpowers/worktrees/ian/sdd-0001-p0-local-creature-proof`
- 主仓库 `main` 尚未包含 0001 实现。
- 0001 worktree 有未提交实现文件。
- 自动验证通过：frontend typecheck、frontend build、frontend tests、Rust check、Rust fmt。
- 0001 verification 存在至少两个需要收口的问题：窗口位置验收口径冲突、位置持久化未完成端到端验证。

## 检查项

| 检查项 | 命令 / 方法 | 结果 | 备注 |
| --- | --- | --- | --- |
| 主仓库状态 | `git status --short --branch` | 通过 | `main` 干净，未包含 0001 实现。 |
| worktree 列表 | `git worktree list --porcelain` | 通过 | 找到 `sdd/0001-p0-local-creature-proof` worktree。 |
| 0001 worktree 状态 | `git status --short --branch` | 通过 | 存在未提交 app 实现和 SDD 修改。 |
| Frontend typecheck | `npm run desktop:typecheck` | 通过，exit 0 | 在 0001 worktree 执行。 |
| Frontend build | `npm run desktop:build` | 通过，exit 0 | 在 0001 worktree 执行。 |
| Frontend unit tests | `npm run desktop:test` | 通过，exit 0 | 1 个 test file，2 个 tests passing。 |
| Rust check | `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` | 通过，exit 0 | 在 0001 worktree 执行。 |
| Rust format | `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml --check` | 通过，exit 0 | 在 0001 worktree 执行。 |

## 验收标准结果

- [ ] 0001 实现所在 worktree 的状态被明确记录：已提交、已合并，或仍为待合并。
- [ ] `node_modules`、`dist`、`target` 等生成目录不会被提交。
- [x] frontend typecheck 通过。
- [x] frontend build 通过。
- [x] frontend unit tests 通过。
- [x] Rust check 通过。
- [x] Rust format check 通过。
- [ ] 0001 verification 中的“右下角窗口”与实际窗口行为不再矛盾。
- [ ] 位置持久化若被勾选完成，则 verification 中有实际验证记录；否则保持未勾选并说明原因。
- [ ] `generated.ts` 的状态被明确记录：已由 Rust 生成，或作为 P0 placeholder 并有后续任务。
- [ ] 0002 不引入任何 P0 非目标功能。

## 失败或缺口

- 0001 仍在独立 worktree 的未提交状态，尚未合并到 `main`。
- 0001 verification 勾选“右下角窗口”，但缺口说明当前为 360x360 居中窗口。
- 0001 position persistence 仍未完成端到端验证。
- `generated.ts` 仍为 placeholder，需要明确是否在 0002 接通生成或延后。

## 后续

用户确认后：

1. 决定 0002 是否修复右下角窗口，还是正式接受居中窗口为 P0 临时形态。
2. 决定位置持久化是否必须在 0002 中完成。
3. 决定 `ts-rs` 生成是否进入 0002。
4. 按 `plan.md` 执行验收收口。

