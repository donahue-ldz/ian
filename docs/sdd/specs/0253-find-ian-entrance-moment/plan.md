# 0253 · 实现计划

## 实现步骤

1. 确认 0191 找回 Ian 基础链路已验收或记录缺口。
2. 为 `find_ian` 事件增加 moment 输出测试。
3. 根据可见状态选择“移动入场”或“原地回应”。
4. 增加连续触发降级和冷却。
5. 桌面端验证快捷键、设置入口和窗口位置。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/behavior_engine.rs`
- `apps/desktop/src/state/useIanActions.ts`
- `apps/desktop/src/renderer/ianStage.css`
- `docs/sdd/specs/0191-find-ian-global-shortcut/verification.md`

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml find_ian
PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:test
PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:tauri -- dev
```

## 风险和回滚

- 风险：找回动画太慢影响实用性。入场总时长应短。
- 回滚：保留基础找回，关闭入场 moment。
