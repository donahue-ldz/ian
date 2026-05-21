# Spec: Local State 与基础设置持久化

## 状态

草稿，等待用户确认。

## 背景

0001 已建立 config / SQLite skeleton，并具备基础 position/config 保存能力，但端到端位置持久化仍未完成验收。P0 需要最小 local-first 状态能力，以支撑“长期住在桌面里”的感觉。

0005 聚焦本地状态和基础设置持久化，不做长期记忆系统。

## 目标

- 明确 P0 需要持久化的最小状态。
- 完成基础 position / config 的读写与重启恢复。
- 保持 SQLite / repository skeleton，为后续 mood、bond、reminder、memory 做准备。
- 提供可复现的持久化验证方式。

## 当前阶段

```txt
P0 / MVP + v0.1.x Product Iteration
```

## 产品范围

- 保存 Ian 位置。
- 保存 active pet / active resource pack。
- 保存 behavior mode 默认值。
- 应用重启后恢复上述配置。

## 非目标

- 不做长期记忆。
- 不做 Mood/Bond 历史。
- 不做提醒记录 UI。
- 不做多宠物管理。
- 不做云同步。
- 不做账号系统。

## 用户体验

用户拖动 Ian 或关闭重启应用后，Ian 应尽量回到上次位置或配置状态。

如果位置无法恢复，应使用安全默认位置，并记录原因。

## 架构约束

- local-first。
- 配置可放在 `~/.ian/config.toml`。
- SQLite 保留 migration / repository skeleton。
- React 不直接写长期状态；持久化通过 Tauri command / Rust Core / StorageService。
- 不读取高敏数据。

## 数据与协议变化

允许调整：

- `IanState.position`
- `save_window_position`
- config schema
- storage repository skeleton
- migration skeleton

不新增高敏 Adapter。

## 隐私与安全

只写入本地 `~/.ian`。

不得写入：

- code content
- clipboard
- private chat content
- screen OCR
- terminal output

## 验收标准

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

## 验证方式

```bash
npm run desktop:typecheck
npm run desktop:build
cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml
```

手动或脚本检查：

- 删除或备份 `~/.ian` 后启动 app。
- 确认 `config.toml` 和 `ian.db` 创建。
- 保存 position。
- 重启 app。
- 确认 position 恢复或 fallback 行为符合记录。

## 开放问题

- P0 位置持久化是否存窗口左上角，还是存 Ian 在窗口内的位置？
- 是否需要为 persistence 增加 Rust 单元测试，还是 P0 先使用 smoke test？

