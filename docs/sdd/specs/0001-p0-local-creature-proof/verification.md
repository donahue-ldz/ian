# 验证记录: P0 本地数字生命验证版

## Spec

`docs/sdd/specs/0001-p0-local-creature-proof/spec.md`

## 状态

草稿。尚未开始实现。

## 验证摘要

当前 packet 正在等待用户确认，尚未开始产品实现，因此还没有运行产品验证。

## 检查项

| 检查项 | 命令 / 方法 | 结果 | 备注 |
| --- | --- | --- | --- |
| SDD packet 存在 | `test -f spec.md && test -f plan.md && test -f decisions.md && test -f verification.md` | 待运行 | packet 创建后执行。 |
| P0 app 启动 | manual `npm run tauri dev` | 未运行 | 需要先实现。 |
| Frontend typecheck | `npm run typecheck` | 未运行 | 需要先实现。 |
| Frontend build | `npm run build` | 未运行 | 需要先实现。 |
| Rust check | `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` | 未运行 | 需要先实现。 |
| Click bubble smoke test | 手动 app 交互 | 未运行 | 需要先实现。 |
| Double-click run-around smoke test | 手动 app 交互 | 未运行 | 需要先实现。 |
| Position persistence smoke test | drag 后重启 app | 未运行 | 需要先实现。 |

## 验收标准结果

- [ ] 仓库中存在 `apps/desktop` 下的 Tauri v2 + React + TypeScript 桌面应用骨架。
- [ ] 应用可以通过文档化的 dev command 在 macOS 上启动。
- [ ] 桌面右下角附近出现透明 always-on-top Ian 窗口。
- [ ] Ian 从 Resource Pack 目录或 placeholder Resource Pack 渲染，路径位于 `public/resources/pets/ian-alpaca`。
- [ ] Ian 至少可以播放 idle 和一个 active animation。
- [ ] 点击 Ian 后，前端发出 `MouseClick` 风格的 `IanEvent`，Rust Core 返回 `IanAction`，React 渲染气泡。
- [ ] Demo Dialogue 不依赖网络或 API key，也能返回短小、有角色感的回复。
- [ ] 双击 Ian 后，前端发出 `MouseDoubleClick` 风格的 `IanEvent`，Rust Core 返回 `BehaviorRunAround` 或等价动作，React 播放 run 行为。
- [ ] 基础位置或配置可以在应用重启后恢复。
- [ ] Rust 定义 `IanEvent`、`IanAction`、`IanState` 的源头协议类型。
- [ ] 前端不拥有核心行为决策，只负责渲染 Rust Core 输出的 action。
- [ ] Adapter、storage、dialogue provider、behavior policy、security skeleton 存在，但不暴露未来阶段用户可见功能。
- [ ] P0 未实现 Git、全局键盘监听、完整 Mood/Bond、长期记忆、Feishu、Pet Visit、plugin system 或 window-aware behavior。

## 失败或缺口

尚未开始实现。

## 后续

用户确认后：

1. 按 `plan.md` 实现。
2. 运行脚手架确定后的验证命令。
3. 将实际命令输出和手动 smoke test 结果更新到本文件。

