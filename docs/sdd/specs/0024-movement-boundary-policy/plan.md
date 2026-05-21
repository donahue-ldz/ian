# 实现计划: Movement Boundary Policy

## 对应规格

`docs/sdd/specs/0024-movement-boundary-policy/spec.md`

## 实现步骤

1. 为边界裁剪写 Rust 失败测试，覆盖负坐标、超右下角和不同 behavior mode。
2. 新增 movement boundary policy 模块或放入现有 behavior policy。
3. 在 run-around 和 roam 输出 movement 前调用边界策略。
4. 如需前端提供屏幕尺寸，只传递低敏 screen/work-area 摘要。
5. 补充 source scan 和边缘位置 smoke。

## 预计改动文件

- `apps/desktop/src-tauri/src/domain/behavior/behavior_policy.rs`
- `apps/desktop/src-tauri/src/domain/behavior/movement_boundary_policy.rs`
- `apps/desktop/src-tauri/src/protocol/state.rs`
- `apps/desktop/src/lib/tauriBridge.ts`
- `docs/sdd/specs/0024-movement-boundary-policy/*`

## 接口 / 兼容性

如果新增 bounds 字段，需要保持旧 config 默认值可恢复。只允许低敏尺寸数据。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
npm run desktop:test
npm run desktop:typecheck
```

## 风险和回滚

不同平台坐标系可能不一致。回滚方式是先限制为当前窗口附近的小范围移动，暂不依赖完整屏幕尺寸。
