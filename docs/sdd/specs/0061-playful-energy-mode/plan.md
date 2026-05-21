# 实现计划: Playful Energy Mode

## 对应规格

`docs/sdd/specs/0061-playful-energy-mode/spec.md`

## 实现步骤

1. 为 playful energy 配置写默认值和持久化测试。
2. 在 BehaviorPolicy 中接入高能强度判断。
3. 在设置面增加低干扰控制项。
4. 确认安静模式和用户 active interaction 优先级更高。
5. 记录验证结果。

## 预计改动文件

- `apps/desktop/src-tauri/src/storage/*`
- `apps/desktop/src-tauri/src/domain/behavior/*`
- `apps/desktop/src/renderer/Settings*`
- `apps/desktop/src/protocol/generated.ts`
- `docs/sdd/specs/0061-playful-energy-mode/*`

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml storage
npm run desktop:test
npm run desktop:typecheck
```

## 风险和回滚

高能控制过早暴露可能让设置变复杂。回滚方式是保留配置字段，暂时放入高级设置。
