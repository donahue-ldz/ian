# 0262 · 实施计划

## 步骤

1. 阅读 `docs/sdd/moment-desktop-acceptance.md` 和 0252-0260 的 `verification.md`。
2. 启动真实 Tauri 桌面壳，确认 Ian 可见、可交互、无明显启动错误。
3. 逐项执行 Moment 验收清单。
4. 将现象拆成可复现条目，记录触发条件、实际结果、影响范围。
5. 输出后续调参优先级，供 0263-0269 使用。

## 预计改动文件

- `docs/sdd/specs/0262-moment-experience-baseline-acceptance/verification.md`
- 如发现清单缺项，可补充 `docs/sdd/moment-desktop-acceptance.md`

## 受影响接口或模块

不修改接口或代码模块。

## 兼容性说明

文档和验收记录不影响运行时兼容性。

## 验证命令或手动检查

- `npm run tauri --workspace @ian/desktop -- dev --config '{"build":{"beforeDevCommand":""}}' --no-dev-server-wait`
- `npm run desktop:test`
- `npm run desktop:typecheck`

## 风险和回滚

- 风险：人工验收结果受环境影响。
- 缓解：记录窗口尺寸、屏幕数量、触发步骤和失败现象。
- 回滚：仅文档记录，无代码回滚需求。
