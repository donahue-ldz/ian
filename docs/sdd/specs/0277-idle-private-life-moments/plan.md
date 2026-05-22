# 0277 · 实施计划

## 步骤

1. 依赖 0270-0272 或确认已有等价能力。
2. 设计 3 类 private life story：peek_around、tiny_patrol、pretend_innocent。
3. 在 Rust Core 中接入 Life Drive 和冷却预算。
4. React 复用 existing animation / movement / bubble 执行。
5. 增加 DND / quiet / reduced motion / active input 抑制测试。
6. 真实桌面诊断触发与 10-20 分钟观察。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/moment_orchestrator.rs`
- `apps/desktop/src-tauri/src/domain/behavior/life_drive.rs`
- `apps/desktop/public/resources/pets/ian-adventurer/animations.json`
- `apps/desktop/src/renderer/IanStage.tsx`
- `apps/desktop/src/renderer/ianStage.css`
- `docs/sdd/specs/0277-idle-private-life-moments/verification.md`

## 受影响接口或模块

- Rare Idle Surprise Moment
- Life Drive
- Resource pack semantic animations

## 兼容性说明

如果资源包缺少专用动画，使用现有语义动画降级。

## 验证命令或手动检查

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm run desktop:test`
- `npm run desktop:typecheck`
- 真实 Tauri 桌面 long-run 观察。

## 风险和回滚

- 风险：idle 私生活过于频繁。
- 缓解：默认低频、强预算、可关闭。
- 回滚：关闭 private life story 类型。
