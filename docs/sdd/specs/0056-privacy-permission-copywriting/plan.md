# 实现计划: Privacy Permission Copywriting

## 对应规格

`docs/sdd/specs/0056-privacy-permission-copywriting/spec.md`

## 实现步骤

1. 盘点所有敏感 capability 和现有 Security Gate 行为。
2. 为每项能力编写中文短文案。
3. 将文案接入设置隐私区和授权入口。
4. 补充权限关闭后的拒绝测试。
5. 做文案和实现一致性审查。

## 预计改动文件

- `apps/desktop/src/renderer/Settings*`
- `apps/desktop/src/renderer/privacy*`
- `apps/desktop/src-tauri/src/security/*`
- `docs/codex/*`
- `docs/sdd/specs/0056-privacy-permission-copywriting/*`

## 接口 / 兼容性

不改变默认权限状态。新增 metadata 时旧配置不需要迁移。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml security
npm run desktop:test
npm run desktop:typecheck
```

## 风险和回滚

文案过长会让设置拥挤。回滚方式是保留三段结构，但默认折叠“不会读取”的详细列表。
