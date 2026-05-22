# 0261 · 实现计划

## 实现步骤

1. 读取 `AGENTS.md`、`docs/codex/ian-current-stage.md`、`docs/codex/ian-architecture-rules.md`、`docs/codex/sdd-workflow.md` 和 `ian.md` 相关章节。
2. 新增 `docs/codex/ian-moment-system.md`，写明 Moment System 技术方案。
3. 更新 `docs/codex/ian-current-stage.md`，把 Moment System 放入 v0.1.x 体验增强方向。
4. 更新 `docs/codex/ian-architecture-rules.md`，加入 Moment Orchestrator 归属和 React 边界。
5. 更新 `ian.md`，在 Behavior System / v0.1.x 附近加入短索引。
6. 更新本 SDD 的 `decisions.md` 和 `verification.md`。
7. 运行文档检查命令。

## 预计改动文件

- `docs/codex/ian-moment-system.md`
- `docs/codex/ian-current-stage.md`
- `docs/codex/ian-architecture-rules.md`
- `ian.md`
- `docs/sdd/specs/0261-moment-system-architecture-doc/*`

## 受影响接口或模块

- 仅文档；不影响 Rust / TypeScript 编译产物。

## 兼容性说明

- 不改变 P0 用户可见范围。
- 不改变现有 SDD 编号。
- 不把 Moment System 实现状态标记为完成；只记录技术方向和边界。

## 验证命令

```bash
rg -n "Moment System|Moment Orchestrator|ian-moment-system|0252|0260" docs/codex ian.md docs/sdd/specs/0261-moment-system-architecture-doc
rg -n "代码正文|剪贴板|屏幕 OCR|全局键盘文本|React 只" docs/codex/ian-moment-system.md docs/codex/ian-architecture-rules.md
```

## 风险和回滚

- 风险：文档让 Moment System 看起来像 P0 必做范围。
  - 应对：明确其属于 v0.1.x Life Feel 增强，不扩大 P0。
- 风险：后续实现把 Moment 写进 React。
  - 应对：架构文档明确 Rust Core / BehaviorPolicy 归属。
- 回滚：删除新增 `ian-moment-system.md` 和三个索引段落。
