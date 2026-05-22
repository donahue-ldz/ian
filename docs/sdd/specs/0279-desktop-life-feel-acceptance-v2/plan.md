# 0279 · 实施计划

## 步骤

1. 阅读 0270-0278 的 spec 和 verification。
2. 新增或更新生命感二阶段桌面验收清单。
3. 为每个能力写清触发方法、预期表现、失败判定和降级检查。
4. 启动真实 Tauri 桌面壳，按清单验收。
5. 记录结果、失败项、剩余风险和下一轮优先级。

## 预计改动文件

- `docs/sdd/life-feel-acceptance-v2.md`
- `apps/desktop/src/acceptance/lifeFeelAcceptance.test.ts`
- `docs/sdd/specs/0279-desktop-life-feel-acceptance-v2/verification.md`

## 受影响接口或模块

- 文档验收清单
- 前端 acceptance 文档保护测试
- 真实桌面 QA 流程

## 兼容性说明

验收文档不影响运行时兼容性。

## 验证命令或手动检查

- `rg -n "0270|0271|0272|0273|0274|0275|0276|0277|0278" docs/sdd/life-feel-acceptance-v2.md`
- `npm run desktop:test`
- 真实 Tauri 桌面完整验收。

## 风险和回滚

- 风险：体验验收过于主观。
- 缓解：每个主观判断都绑定具体触发、观察和失败条件。
- 回滚：保留 0260 Moment 验收清单作为基础验收。
