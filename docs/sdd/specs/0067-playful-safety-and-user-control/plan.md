# 实现计划: Playful Safety And User Control

## 对应规格

`docs/sdd/specs/0067-playful-safety-and-user-control/spec.md`

## 实现步骤

1. 定义 playful safety gate 的判断顺序。
2. 增加关闭、暂停和取消的配置或事件。
3. 在 action executor 中支持高能行为取消和过期。
4. 设置面接入用户控制。
5. 补充 policy、前端和 smoke 验证。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/*`
- `apps/desktop/src-tauri/src/protocol/event.rs`
- `apps/desktop/src/renderer/Settings*`
- `apps/desktop/src/state/*`
- `docs/sdd/specs/0067-playful-safety-and-user-control/*`

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
npm run desktop:test
npm run desktop:typecheck
```

## 风险和回滚

取消逻辑可能影响普通 run-around。回滚方式是先只取消 zoomies，不改变 0022 的短 run-around。
