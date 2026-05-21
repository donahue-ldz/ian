# Spec: Resource Pack Renderer

## 状态

已确认，正在实现。

## 背景

0001 已建立 `apps/desktop`、Tauri shell、Rust Core Runtime、协议骨架，以及 `public/resources/pets/ian-alpaca` Resource Pack 目录。

当前前端虽然会加载 Resource Pack manifest，但 Ian 的主要可见形象仍由 CSS 形状绘制，`sprite.svg` 没有作为动画帧资源真正驱动渲染。0002 的目标是把渲染层从 placeholder creature 推进到真正的 Resource Pack sprite-sheet renderer。

## 目标

- 前端从 `pet.json`、`animations.json` 和 sprite asset 渲染 Ian。
- `AnimationPlayer` 根据 manifest 的 `frames`、`fps`、`loop` 计算当前帧。
- `IanSprite` 使用 Resource Pack sprite sheet 显示 idle / walk / run / happy 等动画帧。
- CSS creature 只作为资源加载失败时的 fallback，不再是正常路径。

## 当前阶段

```txt
P0 / MVP + v0.1 Architecture Baseline
```

## 产品范围

用户可见范围：

- Ian 仍是一个小型桌面生命。
- Ian 仍只展示 0001 已允许的 P0 行为：idle、click happy、double-click run。
- Ian 的视觉来自默认 `ian-alpaca` Resource Pack sprite sheet。

工程范围：

- 保持 `public/resources/pets/ian-alpaca` 作为默认资源包路径。
- 增强前端 resource loading 和 animation frame selection。
- 添加最小自动化测试覆盖动画帧计算。

## 非目标

本 packet 不实现：

- 多宠物选择 UI。
- 资源包市场、资源包导入、资源包权限。
- 声音播放。
- PixiJS / Canvas / WebGL。
- 完整美术资产系统。
- Mood/Bond 影响动画选择。
- BYOM / LLM。

## 用户体验

运行桌面 app 后，Ian 应从 `ian-alpaca` resource pack 的 sprite sheet 中显示，而不是由硬编码 CSS 拼图组成。

点击 Ian 后：

- Rust Core 仍输出 `SpeechShow` 和 `AnimationPlay("happy")`。
- 前端播放 Resource Pack 中 `happy` 对应帧。

双击 Ian 后：

- Rust Core 仍输出 `BehaviorRunAround`。
- 前端播放 Resource Pack 中 `run` 对应帧。

## 架构约束

- Rust Core 仍是行为决策源。
- React 只执行 `IanAction` 并根据 `IanViewState.animation.name` 选择 Resource Pack 动画。
- Resource Pack manifest 描述动画帧，不把动画帧写死在 React 组件里。
- 如果 manifest 缺少请求动画，前端 fallback 到 `idle`。
- 如果 sprite asset 未加载，前端可以使用 CSS fallback 以保证 P0 可见。

## 数据与协议变化

不修改 Rust 协议。

前端 resource 类型允许补充：

- sprite URL
- frame width
- frame height
- scale
- animation frames
- animation fps
- loop

默认 `sprite.svg` 可替换为 sprite sheet 风格的 SVG asset；路径仍由 `pet.json.sprite` 指定。

## 隐私与安全

0002 只读取 app bundled public resource files，不读取用户文件、不访问网络、不请求权限。

## 验收标准

- [ ] `IanSprite` 正常路径使用 Resource Pack sprite asset 渲染，而不是 CSS 拼图。
- [ ] `AnimationPlayer` 或等价模块根据 manifest 的 `frames`、`fps`、`loop` 计算当前帧。
- [ ] 请求不存在的 animation 时 fallback 到 `idle`。
- [ ] `idle`、`happy`、`run` 至少能映射到不同 sprite sheet frame。
- [ ] Resource Pack manifest 仍位于 `apps/desktop/public/resources/pets/ian-alpaca`。
- [ ] 前端测试覆盖 animation frame selection。
- [ ] `npm run desktop:test`、`npm run desktop:typecheck`、`npm run desktop:build` 通过。
- [ ] `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` 通过。

## 验证方式

- TDD red/green：先添加 animation frame selection 测试并确认失败，再实现。
- 自动化命令：
  - `npm run desktop:test`
  - `npm run desktop:typecheck`
  - `npm run desktop:build`
  - `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- 可选 smoke：
  - `npm run desktop:tauri -- dev`
  - click 后观察 happy sprite frame。
  - double-click 后观察 run sprite frame。
