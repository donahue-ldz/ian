# 实现计划: Developer Rhythm Progressive Disclosure

## 对应规格

`docs/sdd/specs/0057-developer-rhythm-progressive-disclosure/spec.md`

## 实现步骤

1. 审查首次体验和设置页中所有 Developer Rhythm 暴露点。
2. 将开发者节奏入口后置到高级、隐私或可选引导。
3. 接入 onboarding flag，但不把它当作权限授权。
4. 确认未授权时无开发者节奏空状态和反应。
5. 补充测试与 smoke 截图。

## 预计改动文件

- `apps/desktop/src/renderer/*Onboarding*`
- `apps/desktop/src/renderer/Settings*`
- `apps/desktop/src-tauri/src/security/*`
- `docs/sdd/specs/0057-developer-rhythm-progressive-disclosure/*`

## 接口 / 兼容性

Developer Rhythm capability 字段不变。新增 onboarding flag 缺失时按“未看过引导”处理。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml security
npm run desktop:test
npm run desktop:typecheck
```

## 风险和回滚

入口后置过深会影响开发者发现。回滚方式是在设置首页保留一行低干扰入口，但不在桌面首屏展示。
