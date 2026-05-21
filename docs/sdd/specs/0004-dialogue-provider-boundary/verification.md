# 验证记录: Dialogue Provider 边界收敛

## Spec

`docs/sdd/specs/0004-dialogue-provider-boundary/spec.md`

## 状态

草稿。尚未实现。

## 验证摘要

暂无验证结果。

## 检查项

| 检查项 | 命令 / 方法 | 结果 | 备注 |
| --- | --- | --- | --- |
| Frontend typecheck | `npm run desktop:typecheck` | 未运行 | 待实现后执行。 |
| Frontend build | `npm run desktop:build` | 未运行 | 待实现后执行。 |
| Frontend tests | `npm run desktop:test` | 未运行 | 待实现后执行。 |
| Rust check | `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` | 未运行 | 待实现后执行。 |

## 验收标准结果

- [ ] Demo Dialogue 通过 provider boundary 调用。
- [ ] DialoguePolicy 对回复长度和角色语气有明确约束。
- [ ] 点击回复和 `DialogueUserMessage` 回复都经过 Rust Core。
- [ ] React 只渲染 `SpeechShow`，不生成核心回复。
- [ ] 默认回复不依赖网络或 API key。
- [ ] 回复不会出现“作为 AI 助手”等 assistant 话术。
- [ ] BYOM 仅保留接口或后续记录，不作为 0004 用户可见功能。
- [ ] Rust check 通过。
- [ ] 前端 typecheck / build 通过。

## 失败或缺口

尚未实现。

## 后续

用户确认后按 `plan.md` 执行。

