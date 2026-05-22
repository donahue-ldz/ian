# 0273 · 实施计划

## 步骤

1. 依赖 0270-0272 完成或确认已有等价能力。
2. 梳理现有 pointer chase / curiosity Moment。
3. 将 chase 输出改为 micro story：discover、pause、chase、settle。
4. 接入 Life Drive，让 curiosity / energy 影响触发概率和动作强度。
5. 增加边界、冷却、快速离开和 reduced motion 测试。
6. 真实桌面使用鼠标反复触发并记录手感。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/moment_orchestrator.rs`
- `apps/desktop/src-tauri/src/domain/behavior/moment_story.rs`
- `apps/desktop/src-tauri/src/domain/behavior/life_drive.rs`
- `apps/desktop/src/renderer/IanStage.tsx`
- `apps/desktop/src/renderer/ianStage.css`
- `docs/sdd/specs/0273-cursor-tease-and-chase-v2/verification.md`

## 受影响接口或模块

- Pointer Curiosity Moment
- Micro Story Framework
- Motion Feel Layer

## 兼容性说明

正式模式继续遵守已有冷却和预算；诊断模式单独触发。

## 验证命令或手动检查

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm run desktop:test`
- `npm run desktop:typecheck`
- 真实 Tauri 桌面 pointer chase 验收。

## 风险和回滚

- 风险：追鼠标过度打扰。
- 缓解：短距离、短时长、强冷却。
- 回滚：恢复 0265 的单段追随策略。
