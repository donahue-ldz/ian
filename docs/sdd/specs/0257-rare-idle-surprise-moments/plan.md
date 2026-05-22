# 0257 · 实现计划

## 实现步骤

1. 定义 rare idle moment 池和触发预算。
2. 增加 TimeTick 下触发 / 不触发测试。
3. 接入资源包语义动画 fallback。
4. 桌面 idle 观察并记录。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/behavior_engine.rs`
- `apps/desktop/src-tauri/src/domain/behavior/*`
- `apps/desktop/src/resources/resourceLoader.ts`

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml idle
PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:test
PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:tauri -- dev
```

## 风险和回滚

- 风险：低频事件仍显得打扰。降低预算或默认关闭。
- 回滚：关闭 rare idle moment。
