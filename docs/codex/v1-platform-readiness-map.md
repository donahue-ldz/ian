# v1 Platform Readiness Map

v1 平台化前置条件：

| 能力 | 前置条件 |
| --- | --- |
| Plugin manifest | manifest schema、权限声明、默认关闭、未知权限拒绝 |
| Local plugin permission | 权限中心授权、撤销传播、Security Gate 检查 |
| Resource pack import | manifest 校验、脚本拒绝、远端引用拒绝、静态资源复制 |
| Optional sound pack | 默认静音、用户确认、音量上限、本地资源 |
| Multi-monitor roaming | 当前安全区域稳定、跨屏 opt-in、边界测试 |
| Multi Ian | `creature_id` 分区、独立权限和状态、全局打扰策略 |
| Social presence | 白名单、清洗、rate limit、Pet Visit 本地动作 allowlist |
| Storage v2 | repository 测试、失败恢复、低敏 payload |
| Telemetry | 默认无遥测、手动导出、明确授权 |

后续建议分批：

1. Memory review 完整 UI 和本地导出。
2. Social Presence 安全闭环，不启用远端内容直显。
3. Resource pack import 工具链。
4. Plugin manifest 和 permission center 联调。
5. 多显示器和多 Ian 产品验证。
