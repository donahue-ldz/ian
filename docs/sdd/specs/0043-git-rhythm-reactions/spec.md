# Spec: Git Rhythm Reactions

## 状态

已实现，待验收。

## 问题 / 目标

0016 已经建立低敏 Git metadata adapter。0043 目标是把授权后的 Git 元数据转成低频、有生命感的 Ian 反应，而不是通知列表。

## 当前产品阶段

v0.2 Developer Rhythm。

## 产品范围

- 对 clean/dirty、branch change、commit seen 等低敏事件做轻量反应。
- 反应包括短句、happy、短暂靠近或安静观察。
- 所有 Git 反应受 cooldown 控制。
- dirty 状态不应被评价或催促。

## 明确不做什么

- 不读取 diff。
- 不读取代码正文。
- 不读取 commit message 全文。
- 不执行 git 写操作。
- 不做代码质量建议。

## 用户体验

Ian 偶尔注意到“你刚提交了”“项目有点乱”这种节奏，但不会催促、不会像 CI 机器人，也不会展示 Git 细节。

## 架构约束

- Git adapter 只输出 IanEvent。
- Behavior / DeveloperRhythmPolicy 决定是否回应。
- React 只执行 IanAction。

## 数据 / 协议变化

复用 `developer.git_status_changed`。如需新增 commit seen，只能包含低敏摘要。

## 隐私与安全边界

payload 只能包含 branch、dirty、短 hash、时间戳等低敏字段。

## 验收标准

- [ ] 授权且 workspace 绑定后 Git 事件可触发低频反应。
- [ ] 未授权或未绑定 workspace 时无用户可见反应。
- [ ] Git 反应有 cooldown，连续 dirty 不刷屏。
- [ ] payload 不含 diff、代码正文、commit message 全文。
- [ ] 测试覆盖 clean、dirty、branch change、cooldown、未授权。

## 验证方式

- Rust adapter / policy tests。
- Source scan。
- 本地 mock Git event smoke。
