# 验证记录: Local State 与基础设置持久化

## Spec

`docs/sdd/specs/0005-local-state-persistence/spec.md`

## 状态

草稿。尚未实现。

## 验证摘要

暂无验证结果。

## 检查项

| 检查项 | 命令 / 方法 | 结果 | 备注 |
| --- | --- | --- | --- |
| Frontend typecheck | `npm run desktop:typecheck` | 未运行 | 待实现后执行。 |
| Frontend build | `npm run desktop:build` | 未运行 | 待实现后执行。 |
| Rust check | `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` | 未运行 | 待实现后执行。 |
| Config init | 检查 `~/.ian/config.toml` | 未运行 | 待实现后执行。 |
| DB init | 检查 `~/.ian/ian.db` | 未运行 | 待实现后执行。 |
| Position persistence | 重启 smoke test | 未运行 | 待实现后执行。 |

## 验收标准结果

- [ ] `~/.ian/config.toml` 可初始化。
- [ ] `~/.ian/ian.db` 可初始化，并运行 migration skeleton。
- [ ] position 可通过 Rust command 保存。
- [ ] app 重启后可恢复 position 或有明确 fallback。
- [ ] active pet / resource pack / behavior mode 有默认值并可恢复。
- [ ] React 不直接写持久化文件。
- [ ] SQLite skeleton 不暴露长期记忆用户功能。
- [ ] Rust check 通过。
- [ ] 前端 typecheck / build 通过。
- [ ] verification 中记录位置持久化 smoke test。

## 失败或缺口

尚未实现。

## 后续

用户确认后按 `plan.md` 执行。

