# 实现计划: Desktop Presence Chrome Reduction

## 对应规格

`docs/sdd/specs/0051-desktop-presence-chrome-reduction/spec.md`

## 实现步骤

1. 审查当前桌面默认 UI，列出持续可见的工具化元素。
2. 将设置入口改为低干扰交互，并补充 tooltip / aria label。
3. 调整透明窗口、气泡和入口层级，避免互相遮挡。
4. 补充前端测试或 smoke 检查，确认设置入口仍可达。
5. 记录截图和验证结果。

## 预计改动文件

- `apps/desktop/src/renderer/*`
- `apps/desktop/src/styles/*`
- `docs/sdd/specs/0051-desktop-presence-chrome-reduction/*`

## 接口 / 兼容性

不改变 Rust 协议和已有配置结构。若新增 UI preference，旧配置缺字段时使用默认低干扰入口。

## 验证命令

```bash
npm run desktop:test
npm run desktop:typecheck
npm run desktop:dev
```

## 风险和回滚

入口过于隐蔽会影响可用性。回滚方式是保留低干扰图标，同时在 hover 或右键菜单中提供明确文字。
