# Spec: Kitten Resource Pack

## Status

Draft

## Context

用户希望把 Ian 现在的形象换成“萌萌哒的小猫咪”，并提供了参考方向：Q 版灰白小猫、黄色大眼睛、白肚皮和白爪、坐姿、开心表情。用户进一步确认采用“新建”方式，而不是在 `ian-alpaca` 资源包中直接替换身份。

当前默认形象由 `apps/desktop/public/resources/pets/ian-alpaca` Resource Pack 提供，前端通过 `pet.json`、`animations.json` 和 `sprite.svg` 渲染 14 帧 sprite sheet。`App.tsx` 当前硬编码加载 `ian-alpaca`。

当前工作树已有大量未提交改动和删除记录，包含上一轮 SDD/实现范围。此任务不得整理、提交或回滚那些既有改动，只能在本规格确认后追加本任务范围内的文件变更。

## Goal

新增一个默认 `ian-kitten` Resource Pack，让 Ian 在 P0 桌面体验中默认显示为参考方向一致但不照抄参考图的 Q 版灰白小猫，并保留现有 idle / walk / sleep / run / happy 动画语义。

## Current Stage

P0 / MVP + v0.1 Architecture Baseline。

## Product Scope

- 新增 `apps/desktop/public/resources/pets/ian-kitten` 资源包。
- 小猫资源包包含 `pet.json`、`animations.json`、`expressions.json` 和 14 帧 `sprite.svg`。
- 小猫视觉方向为 Q 版灰白短毛猫、黄色大眼睛、白色胸口和爪子、开心亲近的表情。
- 桌面端默认加载 `ian-kitten`，浏览器 fallback 状态和 Rust 默认 `IanState` 与该默认值一致。
- 现有 click bubble、Demo Dialogue、double-click run-around、动画播放逻辑继续沿用现有事件和 action 流程。

## Non-Goals

- 不新增资源包管理 UI。
- 不新增多宠物切换 UI。
- 不实现资源包导入、下载、市场或社区资源能力。
- 不改 `IanEvent` / `IanAction` 协议结构。
- 不改 Demo Dialogue 文案和行为决策。
- 不引入真实图片生成运行时依赖。
- 不删除 `ian-alpaca` 资源包；它可继续作为历史/备用资源存在。

## User Experience

启动桌面端或浏览器预览后，默认 Ian 形象显示为新的小猫 sprite。待机时小猫轻微呼吸或眨眼；移动和 run-around 时保持小猫轮廓一致并通过身体位移、尾巴、爪子和表情变化表现动作；点击后仍显示气泡并切到 happy 帧。

## Architecture Constraints

- Resource Pack 仍位于 `apps/desktop/public/resources/pets/<pet-id>/`，由 manifest 驱动渲染。
- React 只负责加载资源包、播放动画和渲染 sprite，不引入行为判断。
- Rust Core 默认 `IanState` 只更新默认 pet/resource pack 标识，不新增行为逻辑。
- 所有交互行为仍通过现有 `IanEvent` 输入和 `IanAction` 输出闭环。
- `ian-alpaca` 不再作为默认形象，但保留在资源目录中，避免破坏已存在引用或回滚路径。

## Data and Protocol Changes

- 新增 Resource Pack 文件：`apps/desktop/public/resources/pets/ian-kitten/*`。
- 更新默认 pet/resource pack id 从 `ian-alpaca` 到 `ian-kitten`：
  - Rust `IanState::default()`
  - 浏览器 fallback `IanState`
  - 前端初始资源包加载路径
  - 相关默认值测试
- 不新增或修改协议字段。
- 不新增数据库 migration。
- 已存在用户本地 `~/.ian/config.toml` 如果仍保存 `ian-alpaca`，本任务不做迁移；P0 默认值和新安装/损坏配置恢复路径使用 `ian-kitten`。

## Privacy and Security

本任务只新增本地静态资源并调整默认资源包 id，不读取敏感数据，不新增网络访问，不新增权限，不改变 Security Gate。

## Acceptance Criteria

- [ ] `apps/desktop/public/resources/pets/ian-kitten/pet.json` 存在，`id` 为 `ian-kitten`，`species` 为 `cat`，并引用同目录的 sprite、animations、expressions 文件。
- [ ] `apps/desktop/public/resources/pets/ian-kitten/animations.json` 定义正数 `frameWidth`、`frameHeight`、`scale`，并包含 `idle`、`walk`、`sleep`、`run`、`happy`。
- [ ] `apps/desktop/public/resources/pets/ian-kitten/sprite.svg` 是 14 帧横向 sprite sheet，尺寸与 animation meta 匹配，并呈现 Q 版灰白小猫、黄色大眼睛、白胸口/白爪、开心亲近表情。
- [ ] 浏览器 fallback 状态、Rust 默认状态和前端首次加载默认资源包都使用 `ian-kitten`。
- [ ] `IanSprite` 继续通过 Resource Pack sprite asset 渲染，不回退到 CSS fallback。
- [ ] 点击 Ian 后仍能显示 bubble 并播放 `happy` 动画；双击 Ian 后仍能播放 `run` 动画。
- [ ] 与默认资源包、动画帧、Rust 默认配置相关的单元测试通过。

## Verification Approach

- 运行前端相关测试：

```bash
npm --workspace apps/desktop test -- --run src/resources/resourceLoader.test.ts src/renderer/AnimationPlayer.test.ts
```

- 运行 Rust 默认配置相关测试：

```bash
cd apps/desktop/src-tauri && cargo test config::tests resource_registry::tests
```

- 启动 Vite 预览并用浏览器检查：

```bash
npm --workspace apps/desktop run dev -- --host 127.0.0.1
```

手动检查 `.ian-sprite-frame` 的 `backgroundImage` 指向 `/resources/pets/ian-kitten/sprite.svg`，初始动画为 `idle`，点击后为 `happy`，双击后为 `run`。

## Open Questions

- 是否要在本任务内把 `ian-alpaca` 的命名测试全部改成不依赖物种名？默认建议：只改默认值相关测试，不重构历史资源包。
