# 实现计划: Playful State And Cooldown Model

## 对应规格

`docs/sdd/specs/0068-playful-state-and-cooldown-model/spec.md`

## 实现步骤

1. 定义 PlayfulState 和 cooldown key。
2. 在 BehaviorEngine / policy 中管理状态切换。
3. 将 zoomies、cute reaction、idle surprise 接入冷却判断。
4. 确认普通点击反馈不被高能冷却误伤。
5. 补充状态机测试和连续触发 smoke。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/*`
- `apps/desktop/src-tauri/src/protocol/state.rs`
- `apps/desktop/src/protocol/generated.ts`
- `docs/sdd/specs/0068-playful-state-and-cooldown-model/*`

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
npm run desktop:test
npm run desktop:typecheck
```

## 风险和回滚

状态机过复杂会拖慢迭代。回滚方式是先实现 cooldown key，不暴露完整 PlayfulState。
