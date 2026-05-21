# 实现计划: Resource Pack Contract

## Spec

`docs/sdd/specs/0008-resource-pack-contract/spec.md`

## 状态

已实现，已验证。

## 概要

把 Resource Pack 的 manifest 读取、校验和 fallback 固化为稳定契约，后续换美术资源时不改渲染架构。

## 步骤

1. 为 resource loader 增加 schema 校验测试。
2. 实现 manifest 校验和错误类型。
3. 明确 idle/default fallback 策略。
4. 补 Rust resource registry 的最小契约测试。
5. 更新文档和 verification。

## 预计文件改动

- `apps/desktop/src/resources/*`
- `apps/desktop/public/resources/pets/ian-alpaca/*`
- `apps/desktop/src-tauri/src/resources/*`
- `docs/sdd/specs/0008-resource-pack-contract/*`

## 接口与边界

不修改行为协议；Resource Pack 仍只影响渲染资源选择。

## 验证命令

```bash
npm run desktop:test
npm run desktop:typecheck
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml resources
```

## 风险

- 过早做复杂 schema 会拖慢资源迭代；0008 只做最小必需字段和清晰 fallback。

## 回滚说明

可保留现有 loader，移除严格校验，继续使用默认资源包。
