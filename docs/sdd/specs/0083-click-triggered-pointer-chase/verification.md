# 0083 验证记录

## 2026-05-21

### TDD 红灯

- `npm run desktop:test -- App.test.ts`：失败，3 个测试失败，原因是 `sendIanClickWithOptionalChase is not a function`。确认测试先于实现覆盖新增行为。

### 针对性验证

- `npm run desktop:test -- App.test.ts`：通过，1 个文件、3 个测试通过。
- `npm run desktop:typecheck`：通过。
- `git diff --check`：通过，无空白错误。

### 回归验证

- `npm run desktop:test`：失败，11 个文件通过、1 个文件失败；失败点是 `src/resources/resourceLoader.test.ts` 读取 `/public/resources/pets/ian-puppy/pet.json` 时文件不存在。该失败不在本次点击追逐调参范围内，未修改资源包。

## 2026-05-21 改为点击后鼠标离开触发

### 根因

- 点击后立即读取鼠标坐标时，鼠标仍在 Ian 身上；0080 追逐逻辑有距离门槛，太近会返回 `blocked_too_close`，即使通过也可能移动很小。
- 点击瞬间发送追逐候选还可能先进入 Rust Core 冷却，让随后鼠标离开时无法观察到追逐。

### TDD 红灯

- `npm run desktop:test -- App.test.ts`：失败，4 个测试失败，原因是 `sendIanClickAndArmChase` / `sendIanLeaveWithArmedChase` 不存在。确认测试先于实现覆盖点击布防、离开发送候选。

### 针对性验证

- `npm run desktop:test -- App.test.ts`：通过，1 个文件、4 个测试通过。
- `npm run desktop:typecheck`：通过。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml pointer_chase`：通过，3 个追逐相关测试通过。
- `git diff --check`：通过，无空白错误。

### 回归验证

- `npm run desktop:test`：通过，12 个文件、63 个测试通过。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：通过，107 个测试通过。

### 剩余风险

- 本次仍未启动真实 Tauri 桌面壳做手动观察。真实效果需要按“点击 Ian -> 鼠标移开 Ian”的动作验证；如果仍不明显，下一步应检查 Rust Core 的距离门槛、冷却状态和窗口坐标是否匹配。

## 2026-05-21 增加短延迟兜底

### 根因补充

- `pointerleave` 绑定在 `.ian-click-target` 上，该透明按钮区域为 192x192。用户从视觉宠物移开时可能仍在按钮区域内，导致离开事件没有触发。
- 如果用户是在浏览器预览里验证，`mouse.chase_candidate` 仍然是 no-op；本次修复面向真实桌面 Tauri 壳。

### TDD 红灯

- `npm run desktop:test -- App.test.ts`：失败，1 个测试失败，原因是 `sendArmedChaseCandidate is not a function`。确认测试覆盖“无需 pointerleave 的延迟兜底候选”。

### 针对性验证

- `npm run desktop:test -- App.test.ts`：通过，1 个文件、5 个测试通过。
- `npm run desktop:typecheck`：通过。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml pointer_chase`：通过，3 个追逐相关测试通过。
- `git diff --check`：通过，无空白错误。

### 回归验证

- `npm run desktop:test`：通过，12 个文件、68 个测试通过。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：通过，107 个测试通过。

### 剩余风险

- 仍未在真实 Tauri 桌面壳里手动观察。若真实桌面仍没有明显追逐，下一步需要给 `mouse.chase_candidate` 返回的 `playful.diagnostic` 暂时显示到调试 UI，确认是否被 `blocked_too_close`、`blocked_too_far`、`blocked_cooldown` 或用户设置拦截。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml pointer_chase`：通过，3 个追逐相关测试通过。
- `npm run desktop:typecheck`：通过。

### 回归验证

- `npm run desktop:test`：通过，12 个文件、61 个测试通过。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`：通过，100 个测试通过。
- `git diff --check`：通过，无空白错误。

### 剩余风险

- 本次没有启动真实 Tauri 桌面壳做手动观察；点击后是否“偶尔”足够明显，需要在真实桌面窗口里体验 25% 概率和 0080 冷却组合后的手感。

## 2026-05-21 调整为 100% 点击候选

### TDD 红灯

- `npm run desktop:test -- App.test.ts`：失败，1 个测试失败，原因是随机值 `0.999` 时未读取桌面鼠标坐标。确认测试覆盖“桌面点击 100% 发送候选”。

### 针对性验证

- `npm run desktop:test -- App.test.ts`：通过，1 个文件、3 个测试通过。
