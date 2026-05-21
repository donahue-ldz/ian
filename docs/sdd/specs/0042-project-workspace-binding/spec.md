# Spec: Project Workspace Binding

## 状态

已实现，待验收。

## 问题 / 目标

Developer Rhythm 需要知道“当前项目”边界，否则 Git 和 build/test 事件没有归属。0042 目标是建立本地项目工作区绑定，只保存低敏路径摘要和项目标识，不读取代码正文。

## 当前产品阶段

v0.2 Developer Rhythm。

## 产品范围

- 用户显式选择或确认当前 workspace。
- 保存 workspace id、根路径、显示名和启用状态。
- Git / build/test adapter 只在绑定 workspace 内生效。
- 支持解绑或暂停 workspace。

## 明确不做什么

- 不扫描整个磁盘。
- 不读取源码内容。
- 不读取 diff。
- 不自动推断 IDE 当前文件。
- 不上传路径或项目信息。

## 用户体验

用户能告诉 Ian：“这个项目可以让你知道一点点开发节奏。”Ian 只在这个项目范围内产生轻量反应。

## 架构约束

- Workspace binding 属于本地配置 / storage。
- Adapter 使用 workspace boundary 过滤事件。
- React 不直接读取 Git 或文件内容。

## 数据 / 协议变化

新增 `developer_workspace` 配置或 repository。路径保存应尽量本地化；如展示路径，应允许用户识别和解绑。

## 隐私与安全边界

只保存用户显式授权的 workspace 根路径和低敏标识，不读取文件内容。

## 验收标准

- [ ] workspace 默认未绑定。
- [ ] 用户可绑定、查看、解绑 workspace。
- [ ] Git / build/test 事件必须带 workspace id 或通过 workspace boundary。
- [ ] 未绑定 workspace 时 adapter 不产生用户可见反应。
- [ ] 测试覆盖绑定、解绑、旧配置默认值和越界事件拒绝。

## 验证方式

- Rust config / adapter tests。
- Frontend settings tests。
- Source scan 检查无源码读取。
