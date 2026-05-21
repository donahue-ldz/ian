# Spec: Git Metadata Adapter

## 状态

已实现，待验收。

## 问题 / 目标

0015 已准备 Developer Rhythm 边界。0016 目标是接入最低敏的 Git 元数据，让 Ian 能理解“当前项目处于什么开发节奏”，但不读取代码正文、不分析 diff、不上传任何仓库信息。

## 当前产品阶段

v0.2 Developer Rhythm。

## 产品范围

- 新增 Git metadata adapter。
- 只读取当前仓库的低敏元数据：branch、dirty 状态、最近一次 commit hash 短值、是否刚 commit/push。
- 将 Git 元数据转换为 `IanEvent`，交给 Rust Core。
- 默认关闭，需要用户在权限设置中显式启用。

## 明确不做什么

- 不读取文件内容。
- 不读取 diff。
- 不分析代码质量。
- 不执行 git 写操作。
- 不自动 commit、push、checkout。

## 用户体验

启用后，Ian 可以对“切分支”“提交成功”“工作树很乱”等轻量开发节奏做短反应，但不展示代码内容。

## 架构约束

- Adapter 只负责收集 Git 元数据并转换为 `IanEvent`。
- Security Gate 必须检查权限。
- BehaviorEngine 决定如何反应。
- React 不直接调用 Git。

## 数据 / 协议变化

可新增 Git metadata 事件，例如 `developer.git_status_changed`、`developer.git_commit_seen`。事件 payload 只能包含低敏元数据。

## 隐私与安全边界

默认关闭。启用后只读取本地仓库元数据，不读取代码正文、diff、远程凭据或 commit message 全文。

## 验收标准

- [ ] Git adapter 默认关闭，未授权时 Security Gate 拒绝事件。
- [ ] 启用后只能读取 branch、dirty 状态和短 commit hash 等低敏元数据。
- [ ] Adapter 不执行 git 写操作。
- [ ] Git 元数据通过 `IanEvent` 进入 Rust Core。
- [ ] 测试覆盖未授权拒绝、授权后事件转换、敏感字段不进入 payload。

## 验证方式

- Rust adapter/security 单元测试。
- `rg` 检查无 diff/code content 读取路径。
- 手动在测试仓库切换 dirty/clean 状态做 smoke。
