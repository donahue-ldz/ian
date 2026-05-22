# 168 · 决策记录

## 2026-05-22

- 本轮按用户授权继续执行剩余 SDD 0160-0189，范围保持为 future skeleton 或 v0.1.x 低敏本地演进，不把 P0 做成完整记忆、社交、插件或平台产品。
- 本地记忆 / Mood / Bond / 亲近边界 只实现最小可验证边界：默认关闭、用户确认、本地优先、低敏摘要、可撤销、无默认联网。
- Memory 只保存低敏 tags 候选；未确认不进入 confirmed 查询，支持删除、过期清理和清空。
- Social / Pet Visit / Group Broadcast 只建立本地策略和 repository skeleton；远端资源、未知动作、未授权消息和超长 payload 默认拒绝。
- Plugin / Resource / Sound / Multi-monitor 只建立校验和策略，不启用插件运行时、远端导入、默认声音或跨屏漫游。
- 平台文档补充在 docs/codex/，明确单实例、storage v2、无默认遥测、贡献者入口和 v1 readiness map。

## Scope Changes

- 0161 的记忆审核界面本轮只作为设置面板中的最小入口和文案，不接真实候选列表；完整交互留到后续。
- 0170-0179 不新增 IanEvent 远端事件，避免未来社交源绕过权限模型；本轮先以 policy/repository 骨架锁定默认拒绝和白名单边界。
- 0180-0184 不执行真实插件、导入文件复制、声音播放或跨屏移动，只提供可测试策略。

## Deferred Work

- 记忆候选完整 UI、诊断导出、Feishu Relay 网络层、Pet Visit 真实播放、插件加载器、资源导入器和多显示器实际漫游均延后到独立 SDD。
