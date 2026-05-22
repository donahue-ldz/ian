# 0094 · 决策记录

## 2026-05-21

- 本 SDD 从 0089 之后追加，当前工作树开始时为干净状态。
- 范围限定在 v0.1 Architecture Baseline hardening，不提前开放 v0.2 / v0.3 用户可见能力。
- 文档中文优先；实现阶段如发生范围或设计变化，必须追加记录。

## Scope Changes

- 2026-05-21：将前端运行时 resource pack 校验从仅检查 `idle` 扩展为检查 `idle` / `walk` / `run` / `zoomies` / `happy` / `rest` / `sleep` 七个语义动画，继续保持纯 JSON/SVG 静态读取，不执行资源包脚本。

## Deferred Work

- 超出本 SDD 的完整平台化、插件化、跨端或高敏感知能力延后处理。
