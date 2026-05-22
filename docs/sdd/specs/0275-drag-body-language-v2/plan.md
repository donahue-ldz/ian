# 0275 · 实施计划

## 步骤

1. 检查当前 IanStage 拖动命中区、pointer capture 和 position persistence。
2. 设计 drag visual states：pickup、carried、about_to_drop。
3. Rust Core 输出 carry / suppress Moment 状态。
4. React 使用 CSS class 表达拖动姿态和轻微延迟感。
5. 增加拖动稳定性和 Moment 抑制测试。
6. 真实桌面连续拖动并记录手感。

## 预计改动文件

- `apps/desktop/src/renderer/IanStage.tsx`
- `apps/desktop/src/renderer/ianStage.css`
- `apps/desktop/src/lib/position.ts`
- `apps/desktop/src-tauri/src/domain/behavior/behavior_engine.rs`
- `apps/desktop/src-tauri/src/domain/behavior/moment_orchestrator.rs`
- `docs/sdd/specs/0275-drag-body-language-v2/verification.md`

## 受影响接口或模块

- Drag Carry Moment
- pointer input handling
- position persistence
- interaction suppression gate

## 兼容性说明

不修改已有持久化位置格式。

## 验证命令或手动检查

- `npm run desktop:test`
- `npm run desktop:typecheck`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- 真实 Tauri 桌面连续拖动验收。

## 风险和回滚

- 风险：视觉延迟让用户感觉拖动不跟手。
- 缓解：延迟只作用于姿态，不改变窗口真实坐标。
- 回滚：关闭拖动姿态 class。
