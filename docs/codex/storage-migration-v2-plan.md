# Storage Migration v2 Plan

v2 migration 目标是为 memory、social、plugin 和 resource ecosystem 做稳定演进，但不提前启用未来产品能力。

已建立的未来表：

- `memory_candidates`
- `social_whitelist`
- `visit_records`
- `adapter_audit_logs`

下一步迁移原则：

- 新表必须有 repository 测试。
- 失败时保持旧 config 和 SQLite 数据可读。
- API key、用户正文、代码正文、私聊内容、剪贴板和窗口标题不得进入迁移数据。
- JSON payload 只保存低敏摘要，不替代核心字段。
- 删除和撤权路径必须可测试。

回滚策略：

- 保留向后兼容字段默认值。
- 新能力默认关闭时，可以保留无害 schema。
- 如 schema 破坏启动，应回退到默认 state 并记录诊断，不删除用户数据。
