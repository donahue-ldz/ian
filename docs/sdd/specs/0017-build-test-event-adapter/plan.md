# 实现计划: Build Test Event Adapter

## Spec

`docs/sdd/specs/0017-build-test-event-adapter/spec.md`

## 状态

已实现，待验收。

## 概要

为 Developer Rhythm 增加低敏 build/test 摘要事件入口，不读取终端全文。

## 步骤

1. 定义 build/test summary protocol。
2. 增加 payload size 和 permission 测试。
3. 实现 mock adapter 或 command 接入口。
4. 在 BehaviorPolicy 中映射 success/failure 轻反应。
5. 做 source scan 和 smoke。

## 预计文件改动

- `apps/desktop/src-tauri/src/protocol/event.rs`
- `apps/desktop/src-tauri/src/security/*`
- `apps/desktop/src-tauri/src/domain/behavior/*`
- `apps/desktop/src-tauri/src/adapters/*`

## 接口与边界

Ian 接收摘要事件，不主动执行构建测试，不读取终端全文。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml protocol
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml security
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
rg -n "stdout|stderr|terminal|Command::new" apps/desktop/src-tauri/src
```

## 风险

- 失败信息可能包含敏感路径或源码片段；只允许短错误类别。

## 回滚说明

禁用 build/test permission 或移除 event handler。
