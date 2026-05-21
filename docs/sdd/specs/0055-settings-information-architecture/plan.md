# 实现计划: Settings Information Architecture

## 对应规格

`docs/sdd/specs/0055-settings-information-architecture/spec.md`

## 实现步骤

1. 盘点当前设置项和对应配置字段。
2. 建立用户心智分组，不改变底层配置来源。
3. 调整设置面布局、分组标题和说明文案。
4. 确认隐私和高级入口清晰可达。
5. 补充测试、截图和响应式检查。

## 预计改动文件

- `apps/desktop/src/renderer/Settings*`
- `apps/desktop/src/renderer/settingsModel*`
- `apps/desktop/src/styles/*`
- `docs/sdd/specs/0055-settings-information-architecture/*`

## 接口 / 兼容性

设置字段保持兼容。布局重组不得改变旧配置读取结果。

## 验证命令

```bash
npm run desktop:test
npm run desktop:typecheck
```

## 风险和回滚

分组过多会让设置显得复杂。回滚方式是保留五类信息架构，但默认只展开常用两类。
