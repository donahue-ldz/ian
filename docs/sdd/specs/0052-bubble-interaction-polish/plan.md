# 实现计划: Bubble Interaction Polish

## 对应规格

`docs/sdd/specs/0052-bubble-interaction-polish/spec.md`

## 实现步骤

1. 梳理当前气泡组件、输入态和 dialogue action 的来源。
2. 增加气泡长度、替换和消退规则。
3. 将用户输入聚焦状态接入 Ian 行为暂停边界。
4. 调整气泡样式，确保短句、输入框和动画不互相挤压。
5. 补充测试和 smoke 记录。

## 预计改动文件

- `apps/desktop/src/renderer/Bubble*`
- `apps/desktop/src/renderer/*Ian*`
- `apps/desktop/src-tauri/src/protocol/*`
- `docs/sdd/specs/0052-bubble-interaction-polish/*`

## 接口 / 兼容性

新增输入态事件时必须保持旧 action 可执行。旧气泡 action 没有显示时长时使用默认时长。

## 验证命令

```bash
npm run desktop:test
npm run desktop:typecheck
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
```

## 风险和回滚

暂停自主动作可能让 Ian 显得过静。回滚方式是只暂停位移和覆盖性气泡，保留轻微待机动画。
