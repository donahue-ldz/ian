# Spec: Playful Expressiveness Acceptance Suite

## 状态

已实现，待用户验收。

## 问题 / 目标

高能卖萌体验需要单独验收，不能只看单测。0070 目标是建立验收套件，确认 Ian 真的具备随机感、满屏乱跑和撒娇卖萌，同时仍然可控、不烦、不越界。

## 产品范围

- 验收 zoomies：触发、路径范围、结束态、取消、边界。
- 验收 cute reaction pack：短句丰富度、去重、连续互动。
- 验收 controlled randomness：固定 seed 可复现，真实运行有变化。
- 验收 safety：off、quiet、拖拽、输入、设置打开时不触发。
- 验收视觉：高能状态可区分，reduced motion 生效。

## 明确不做什么

- 不替代 0061-0069 的单项测试。
- 不要求每次运行都触发自发彩蛋。
- 不做外部服务或云端验收。
- 不验收 Developer Rhythm。

## 用户体验

通过 0070 后，Ian 应该从“克制可用”升级为“会突然可爱一下”：能跑、能撒娇、有变化，但用户仍然掌控它。

## 架构约束

- 自动化覆盖 Rust behavior、scheduler、protocol 和 React action executor。
- 视觉体验用 Browser / Tauri 截图或录屏补充。
- 验收结果写入 `verification.md`，包含命令、截图和人工结论。

## 数据 / 协议变化

不新增产品协议。可新增测试 fixtures、seed 配置和 smoke checklist。

## 隐私与安全边界

验收只使用本地 Ian 事件和测试数据，不读取用户真实应用、键盘、终端或代码内容。

## 验收标准

- [x] zoomies 能在安全边界内跑过多个区域，并在时限内结束。
- [x] cute reaction 至少覆盖多种短句和动作，不连续重复刷屏。
- [x] off / quiet / active interaction 能阻止高能行为。
- [x] 随机测试可复现，真实 smoke 不机械重复。
- [x] reduced motion 和 fallback 资源通过检查。
- [x] 验收记录命令、截图和人工结论。

## 验证方式

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm run desktop:test`
- `npm run desktop:typecheck`
- Browser / Tauri smoke checklist。
