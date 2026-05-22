# 0272 · 实施计划

## 步骤

1. 查看当前 movement action、IanStage 执行和 CSS 动画。
2. 设计最小 motion profile 枚举，保持语义化。
3. Rust Core 在相关 Moment 中输出 profile。
4. React 根据 profile 设置 class、duration、easing、tilt、settle。
5. 增加 reduced motion 降级和连续移动覆盖测试。
6. 真实桌面观察找回、追随、放下三个移动场景。

## 预计改动文件

- `apps/desktop/src-tauri/src/protocol/action.rs`
- `apps/desktop/src/protocol/generated.ts`
- `apps/desktop/src/state/ianActions.ts`
- `apps/desktop/src/renderer/IanStage.tsx`
- `apps/desktop/src/renderer/ianStage.css`
- `docs/sdd/specs/0272-motion-physics-feel-layer/verification.md`

## 受影响接口或模块

- IanAction movement
- React action executor
- IanStage CSS animation

## 兼容性说明

旧 movement action 未带 profile 时使用 `gentle` 默认值。

## 验证命令或手动检查

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm run desktop:test`
- `npm run desktop:typecheck`
- 真实 Tauri 桌面移动手感验收。

## 风险和回滚

- 风险：过多 CSS 动画造成位置不准。
- 缓解：profile 只影响视觉层，不改变最终目标位置。
- 回滚：忽略 profile，恢复普通移动。
