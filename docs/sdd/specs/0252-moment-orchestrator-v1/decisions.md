# 0252 · 决策记录

| 日期 | 决策 | 原因 | 影响 |
| --- | --- | --- | --- |
| 2026-05-22 | 将“惊喜感”定义为 moment 编排问题，而不是继续堆独立动画。 | 当前能力点很多，但缺少有铺垫的短场景。 | 后续 SDD 先补编排层，再补具体瞬间。 |
| 2026-05-22 | Moment budget 和 cooldown 先放在 Rust Core 行为引擎内存中。 | P0/v0.1.x 只需要控制单次桌面运行期的惊喜密度，跨进程持久化会扩大范围。 | 重启后 budget 重置；如后续需要跨会话预算，另开 SDD 设计存储迁移。 |
| 2026-05-22 | Moment 以 `IanAction::PlayfulDiagnostic` 暴露低敏调试信息，不新增协议字段。 | 现有 action 已能表达低敏 reason、kind 和时间，避免为早期 moment 引入额外协议面。 | React 继续只执行 action；后续如需要 richer telemetry 再通过协议 SDD 扩展。 |

## Deferred Work

- 逐项人工视觉验收和动画调参留给后续 UI / asset SDD。
