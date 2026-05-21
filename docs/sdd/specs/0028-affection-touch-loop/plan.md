# 实现计划: Affection Touch Loop

## 对应规格

`docs/sdd/specs/0028-affection-touch-loop/spec.md`

## 实现步骤

1. 为连续点击和 Bond context 写 Rust 失败测试。
2. 在 BondEngine 或 BehaviorPolicy 中增加本地互动窗口统计。
3. 在 BehaviorEngine 中输出亲近反馈、降频或轻微躲开动作。
4. 确认 DialoguePolicy 对亲近短句仍做长度和身份约束。
5. 浏览器 smoke 验证连续点击不刷屏且无数值 UI。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/bond/*`
- `apps/desktop/src-tauri/src/domain/behavior/behavior_engine.rs`
- `apps/desktop/src-tauri/src/domain/behavior/behavior_policy.rs`
- `apps/desktop/src-tauri/src/domain/dialogue/dialogue_policy.rs`
- `apps/desktop/src/renderer/IanStage.tsx`
- `docs/sdd/specs/0028-affection-touch-loop/*`

## 接口 / 兼容性

优先使用现有事件；如果新增 interaction kind，要保持旧点击行为不变。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml bond
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml dialogue
npm run desktop:test
```

## 风险和回滚

亲近反馈容易过度拟人或刷屏。回滚方式是只保留单击反馈和 cooldown，关闭连续互动分支。
