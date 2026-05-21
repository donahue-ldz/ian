# 实现计划: Controlled Randomness Policy

## 对应规格

`docs/sdd/specs/0063-controlled-randomness-policy/spec.md`

## 实现步骤

1. 设计 Rust Core random provider trait 或轻量 helper。
2. 为路径、短句和触发候选接入受控随机。
3. 在测试中提供固定 seed 或固定序列。
4. 移除或禁止 React 行为随机决策。
5. 补充测试和约束文档。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/*`
- `apps/desktop/src-tauri/src/core/*`
- `apps/desktop/src-tauri/src/protocol/*`
- `docs/sdd/specs/0063-controlled-randomness-policy/*`

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml core
npm run desktop:test
```

## 风险和回滚

抽象过重会增加复杂度。回滚方式是先实现局部 seedable helper，不做全局随机框架。
