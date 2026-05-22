# Multi-Ian Architecture Note

Ian 当前只支持单实例。

原因：

- P0 目标是验证一个桌面生命是否足够有存在感。
- 当前 `IanState`、窗口位置、resource pack、记忆候选和互动记录都围绕单个本地 Ian 设计。
- 多实例会放大权限、资源占用、气泡打扰和社交边界风险。

未来如支持多 Ian，应先引入：

- `creature_id` 作为状态、事件、记忆、visit record 和 resource pack 的分区键。
- 每个 Ian 独立的权限、位置、resource pack、personality 和记忆候选。
- 单窗口多 Ian 与多窗口多 Ian 的明确产品选择。
- 全局打扰策略，防止多个 Ian 同时弹气泡或播放声音。

当前 resource switcher 只是切换同一个 Ian 的外观，不代表多实例。
