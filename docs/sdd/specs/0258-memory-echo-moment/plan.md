# 0258 · 实现计划

## 实现步骤

1. 确认 0196-0200 记忆 UI 和 confirmed usage 基础是否完成。
2. 为 confirmed / candidate / deleted 三类记忆写使用测试。
3. 增加低频 memory echo 选择策略。
4. 增加文案安全测试。
5. 桌面验收气泡显示。

## 预计改动文件

- `apps/desktop/src-tauri/src/storage/repositories/memory_repo.rs`
- `apps/desktop/src-tauri/src/domain/dialogue/dialogue_engine.rs`
- `apps/desktop/src-tauri/src/domain/behavior/behavior_engine.rs`

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml memory
PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:test
```

## 风险和回滚

- 风险：用户感觉被监控。只用 confirmed tags，频率极低，可关闭。
- 回滚：关闭 memory echo，保留 memory repository。
