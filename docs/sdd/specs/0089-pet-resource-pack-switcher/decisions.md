# Decisions: Pet Resource Pack Switcher

## Decision Log

| Date | Decision | Reason | Impact |
| --- | --- | --- | --- |
| 2026-05-21 | 将本功能限定为“当前 Ian 的 resource pack 切换”，不做多只宠物共存。 | P0 明确不做 multi-pet / multi-Ian support；用户目标是切换已有宠物外观。 | 可以补齐设置入口和持久化，不引入身份、记忆、窗口和行为实例管理。 |
| 2026-05-21 | Rust command 使用已知内置 id 白名单。 | 避免把任意字符串持久化为资源路径。 | 新增资源包时需要同步更新白名单和前端选项。 |

## Scope Changes

None.

## Deferred Work

- 资源包动态发现。
- 宠物预览图。
- 每个宠物独立身份、偏好、记忆或成长状态。

