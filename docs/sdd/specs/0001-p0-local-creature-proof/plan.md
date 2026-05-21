# 实现计划: P0 本地数字生命验证版

## Spec

`docs/sdd/specs/0001-p0-local-creature-proof/spec.md`

## 状态

已确认，已执行到当前 P0 骨架验证边界；剩余后续项是类型生成链路和拖拽重启端到端验证。

## 概要

创建第一版可运行 Ian 桌面验证，同时保留长期架构骨架：

- 创建 `apps/desktop`。
- 接入 Tauri v2 + React + TypeScript。
- 定义 Rust 源头协议类型。
- 实现最小 Rust Core event / action 闭环。
- 渲染 placeholder / Resource Pack Ian。
- 支持点击气泡、Demo Dialogue、双击乱跑和位置持久化。

## 验收标准映射

| 实现步骤 | 对应验收标准 |
| --- | --- |
| 1. 创建 desktop app 骨架 | app skeleton、launch command |
| 2. 配置 desktop shell | 透明 always-on-top 窗口、右下角位置 |
| 3. 添加协议层 | Rust source types、TS generation path |
| 4. 添加 Rust Core skeleton | Rust 拥有行为决策、event/action 闭环 |
| 5. 添加 Resource Pack skeleton | Ian 从 Resource Pack 或 placeholder 渲染 |
| 6. 添加前端 renderer | idle/active animation、React 渲染 actions |
| 7. 添加互动闭环 | click event/action/bubble、double-click run-around |
| 8. 添加持久化 skeleton | position/config 重启后恢复 |
| 9. 添加未来边界 skeleton | adapter/storage/dialogue/policy/security skeletons |
| 10. 验证 | build/check/manual smoke criteria |

## 步骤

1. 创建仓库结构。
   - 创建 `apps/desktop`。
   - 添加 package metadata 和 scripts。
   - 添加 Tauri v2、React、Vite、TypeScript 基线文件。

2. 配置 desktop shell。
   - 配置透明、无边框、always-on-top 窗口。
   - Ian 默认出现在桌面右下角附近。
   - 添加前端 IPC 所需的基础 app commands。

3. 定义协议层。
   - 添加 Rust `IanEvent`、`IanAction`、`IanState`。
   - 使用 `ts-rs` 添加 TypeScript 生成路径；在 Rust 工具链不可用时添加有明确标注的临时 generated file。
   - 保持 Rust 作为协议源头。

4. 构建最小 Rust Core Runtime。
   - 添加 event bus / runtime entrypoint。
   - 将 frontend commands 转换为 `IanEvent`。
   - 返回或发出 `IanAction`。
   - 如动画或 idle 行为需要，添加 scheduler / tick skeleton。

5. 添加架构 skeleton modules。
   - `adapters`: trait + Time / Mouse / Dialogue minimal adapters。
   - `domain/behavior`: behavior policy boundary。
   - `domain/dialogue`: Demo Dialogue provider boundary。
   - `storage`: config / SQLite / migration / repository skeleton。
   - `security`: permission / sanitizer / rate limiter skeleton。

6. 添加默认 Resource Pack。
   - 创建 `public/resources/pets/ian-alpaca`。
   - 添加 `pet.json`、`animations.json`、`expressions.json`。
   - 添加 placeholder visual asset，并记录临时 placeholder renderer。

7. 构建前端 renderer。
   - 添加 `IanStage`、`IanSprite`、`AnimationPlayer`、`Bubble` 和最小 state hooks。
   - 渲染 Rust Core 输出的 actions。
   - 避免在 React 中写核心行为决策。

8. 实现 P0 互动。
   - click 发送 `MouseClick`；Rust 返回 `SpeechShow` 或 `BubbleOpen` actions。
   - Demo Dialogue 返回本地短句。
   - double-click 发送 `MouseDoubleClick`；Rust 返回 run-around action sequence。
   - drag 更新位置。

9. 实现基础持久化。
   - 本地保存 position / config。
   - 重启后恢复 position / config。
   - 即使 P0 持久化很薄，也保持 SQLite / config skeleton 与未来架构一致。

10. 验证。
    - 安装依赖。
    - 运行前端 typecheck / build。
    - 运行 Rust check。
    - 启动 app。
    - 手动验证 click bubble、double-click run-around 和 restart persistence。
    - 将结果记录到 `verification.md`。

## 预计文件改动

- `apps/desktop/package.json`: frontend scripts 和 dependencies。
- `apps/desktop/src/*`: React app、renderer、bubble、state hooks。
- `apps/desktop/public/resources/pets/ian-alpaca/*`: 默认 Resource Pack。
- `apps/desktop/src-tauri/*`: Tauri 配置和 Rust app。
- `apps/desktop/src-tauri/src/protocol/*`: `IanEvent`、`IanAction`、`IanState`。
- `apps/desktop/src-tauri/src/core/*`: runtime、event bus、action dispatcher、scheduler。
- `apps/desktop/src-tauri/src/domain/*`: behavior 和 dialogue skeleton。
- `apps/desktop/src-tauri/src/adapters/*`: adapter trait 和 minimal adapters。
- `apps/desktop/src-tauri/src/storage/*`: config、database、migration、repository skeleton。
- `apps/desktop/src-tauri/src/security/*`: permission、sanitizer、rate limiter skeleton。
- `docs/sdd/specs/0001-p0-local-creature-proof/decisions.md`: 实现决策记录。
- `docs/sdd/specs/0001-p0-local-creature-proof/verification.md`: 验证结果记录。

## 接口与边界

- Frontend 通过 Tauri commands 发送事件。
- Rust Core 以 `IanEvent` 接收事件。
- Rust Core 产出 `IanAction`。
- Frontend 渲染 `IanAction`，不拥有 behavior policy。
- Resource Pack manifest 描述视觉资源和动画。
- 未来敏感集成仅保留 skeleton，不变成 P0 用户可见功能。

## 验证命令

最终命令可能根据脚手架选择调整。预期检查：

```bash
npm install
npm run typecheck
npm run build
cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml
npm run tauri dev
```

手动检查：

- app 打开透明 always-on-top Ian 窗口。
- click 打开气泡。
- double-click 触发 run-around。
- restart 后恢复 position / config。

## 风险

- Tauri v2 透明窗口行为可能受 macOS 权限和配置影响。
- placeholder 视觉可能不足以传达生命感。
- `ts-rs` 集成可能增加搭建复杂度；如果阻塞 P0，可能需要临时 generated file。
- position persistence 可以先通过 config 实现，同时保留 SQLite skeleton 给后续持久化。

## 回滚说明

这是第一版 app scaffold，回滚相对直接：

- 删除 `apps/desktop`。
- 删除生成的 package lock files。
- 回滚 SDD packet 对应实现改动。

除非产品方向变化，否则保留 SDD 文档。
