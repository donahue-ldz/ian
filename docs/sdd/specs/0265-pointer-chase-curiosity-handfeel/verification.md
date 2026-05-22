# 0265 · 验证记录

## 2026-05-22

### 已执行检查

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 结果：通过。171 个 Rust 测试通过，覆盖 pointer chase、pointer curiosity cooldown、用户交互 gate 和 reduced motion gate。
- `PATH=/opt/homebrew/bin:$PATH npm run test --workspace @ian/desktop -- src/lib/tauriBridge.browserFallback.test.ts`
  - 结果：通过。
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:typecheck`
  - 结果：通过。

## 验收标准结果

- [x] 诊断模式可以稳定触发 `pointer_curiosity`。
- [x] 正式 mouse-near 保留 cooldown 和 Core guardrail。
- [x] Pointer chase 仍有距离边界和 cooldown，不会持续贴着鼠标。
- [x] reduced motion 下追随降级，不主动移动。
- [x] 不记录完整全局轨迹或鼠标下方内容。

## 剩余风险

- 未用真实鼠标做长时间手感观察；本轮以自动行为测试和桌面启动 smoke 作为替代。
