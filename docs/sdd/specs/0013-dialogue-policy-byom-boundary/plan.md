# 实现计划: Dialogue Policy and BYOM Boundary

## Spec

`docs/sdd/specs/0013-dialogue-policy-byom-boundary/spec.md`

## 状态

已实现，待验收。

## 概要

把对话策略从短句裁剪升级为稳定策略边界，并为 BYOM 留好安全配置位置。

## 步骤

1. 增加 DialoguePolicy 测试。
2. 强化输出裁剪、身份约束和 fallback。
3. 定义 BYOM config / provider selector。
4. 接入 secret placeholder，不做默认网络请求。
5. 验证 Demo Dialogue 回归。

## 预计文件改动

- `apps/desktop/src-tauri/src/domain/dialogue/*`
- `apps/desktop/src-tauri/src/desktop/secrets.rs`
- `apps/desktop/src-tauri/src/security/*`
- `apps/desktop/src/lib/tauriBridge.ts`（如有设置入口）

## 接口与边界

Provider 选择在 Rust Core；React 不直接调用外部模型。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml dialogue
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml security
npm run desktop:test
```

## 风险

- BYOM 容易压过 Ian 的生命感；默认必须保持 Demo Provider。

## 回滚说明

保留 Demo Provider，移除 BYOM selector 或让其固定 no-op。
