# 实现计划: Idle Rest Visual Comfort

## 对应规格

`docs/sdd/specs/0058-idle-rest-visual-comfort/spec.md`

## 实现步骤

1. 梳理 idle、rest、sleep 当前状态和动画资源。
2. 定义视觉状态切换规则和互斥条件。
3. 调整微动作动画和 reduced motion / quiet mode 映射。
4. 确认用户交互能自然唤醒或打断休息。
5. 记录 5 分钟 idle smoke。

## 预计改动文件

- `apps/desktop/src-tauri/src/behavior/*`
- `apps/desktop/src/renderer/*Animation*`
- `apps/desktop/resources/*`
- `docs/sdd/specs/0058-idle-rest-visual-comfort/*`

## 接口 / 兼容性

保持已有 idle / sleep 状态兼容。新增 rest visual state 缺失时退化为 idle。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
npm run desktop:test
npm run desktop:typecheck
```

## 风险和回滚

微动作过弱会显得静态。回滚方式是提高眨眼和呼吸频率，但不提高大幅移动频率。
