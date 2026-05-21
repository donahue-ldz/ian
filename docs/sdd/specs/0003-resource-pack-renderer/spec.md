# Spec: Resource Pack 渲染收敛

## 状态

草稿，等待用户确认。

## 背景

0001 已建立 `public/resources/pets/ian-alpaca` 资源包目录，并提供 placeholder 视觉。当前仍存在两个问题：

- Ian 的视觉主要依赖 CSS placeholder，Resource Pack 还没有真正成为渲染源头。
- 后续要支持更多动作、皮肤、sprite sheet 和社区资源，需要先把 Resource Pack 渲染边界收敛清楚。

0003 聚焦 Resource Pack 渲染，不扩展新产品系统。

## 目标

让前端渲染层真正以 Resource Pack 为输入：

- 读取 `pet.json`、`animations.json`、`expressions.json`。
- 支持基于 manifest 的动画选择。
- 支持当前 placeholder asset，同时为后续 sprite sheet 留好接口。
- 让 `IanSprite` / `AnimationPlayer` 的职责更清晰。

## 当前阶段

```txt
P0 / MVP + v0.1.x Product Iteration
```

## 产品范围

- Ian 仍显示为一个简单桌面生命。
- 支持 idle、walk、happy、run、sleep 的 manifest 驱动动画选择。
- 允许继续使用 placeholder visual asset。
- 提升动画和 Resource Pack 加载失败时的降级体验。

## 非目标

- 不做正式美术。
- 不做资源包市场。
- 不做在线下载资源包。
- 不做多宠物选择 UI。
- 不做脚本化资源包。
- 不做声音系统。
- 不做粒子、Live2D、Canvas 或 WebGL 重构。

## 用户体验

用户看到的 Ian 仍然是轻量桌面生命，但动画来源和内部结构应更接近长期设计。

如果资源包加载失败，Ian 应降级到安全 placeholder，而不是空白或崩溃。

## 架构约束

- React 负责渲染和动画播放，不负责行为决策。
- Rust Core 仍通过 `IanAction.AnimationPlay` 决定动画名。
- Resource Pack manifest 只描述视觉资源和动画，不包含行为逻辑。
- 前端可以有资源加载缓存，但不能绕过 `IanAction` 自行决定核心行为。

## 数据与协议变化

预期不新增 `IanEvent` 或 `IanAction`。

允许调整：

- `AnimationName` 类型。
- Resource Pack manifest TypeScript 类型。
- `AnimationPlayer` 的输入结构。

## 隐私与安全

只加载本地打包资源或 public 目录资源。

不加载远程资源。

## 验收标准

- [ ] `IanSprite` / `AnimationPlayer` 的动画输入来自 Resource Pack manifest。
- [ ] `pet.json`、`animations.json`、`expressions.json` 加载失败时有明确 fallback。
- [ ] idle、walk、happy、run、sleep 均能被 manifest 描述并被前端识别。
- [ ] `IanAction.AnimationPlay` 仍是动画切换入口。
- [ ] React 未新增核心行为决策逻辑。
- [ ] 前端 typecheck 通过。
- [ ] 前端 build 通过。
- [ ] 相关单元测试或轻量组件测试覆盖资源加载 / fallback。

## 验证方式

```bash
npm run desktop:typecheck
npm run desktop:build
npm run desktop:test
```

手动检查：

- Ian 正常渲染。
- click 后 happy 动画仍工作。
- double-click 后 run 动画仍工作。

## 开放问题

- 0003 是否继续使用 `sprite.svg`，还是引入最小 bitmap sprite sheet？
- 是否需要在 0003 中补截图验证，还是留到视觉 polish 阶段？

