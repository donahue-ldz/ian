# 0260 · 实现计划

## 实现步骤

1. 汇总 0252-0259 的 moment 验收点。
2. 编写桌面验收清单，包含操作、预期和失败判定。
3. 标记哪些可自动化，哪些需要人工观察。
4. 启动真实 Tauri 桌面壳做一次 smoke。
5. 更新 verification。

## 预计改动文件

- `docs/sdd/moment-desktop-acceptance.md`
- `docs/sdd/specs/0260-moment-desktop-acceptance-suite/verification.md`

## 验证命令

```bash
PATH=/opt/homebrew/bin:$PATH /opt/homebrew/bin/npm run desktop:tauri -- dev
```

## 风险和回滚

- 风险：验收清单过长导致没人执行。保持主路径短，细项分批。
- 回滚：保留最小 smoke 清单。
