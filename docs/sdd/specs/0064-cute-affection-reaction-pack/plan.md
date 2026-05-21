# 实现计划: Cute Affection Reaction Pack

## 对应规格

`docs/sdd/specs/0064-cute-affection-reaction-pack/spec.md`

## 实现步骤

1. 定义 reaction pack 数据结构和中文短句池。
2. 将反应选择接入 affection / playful policy。
3. 增加去重、冷却和场景权重。
4. 接入动作组合和资源 manifest。
5. 做文案审查和连续互动 smoke。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/*`
- `apps/desktop/src-tauri/src/domain/dialogue/*`
- `apps/desktop/resources/*`
- `docs/sdd/specs/0064-cute-affection-reaction-pack/*`

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml dialogue
npm run desktop:test
```

## 风险和回滚

文案过甜会偏离陪伴感。回滚方式是保留反应包结构，收敛短句语气。

## 执行结果

已按本计划完成，验证记录见 `verification.md`。
