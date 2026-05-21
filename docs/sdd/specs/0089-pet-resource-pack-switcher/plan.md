# Plan: Pet Resource Pack Switcher

## 实现步骤

1. 定义内置 resource pack 列表
   - 在前端 resource loader 模块导出内置宠物选项。
   - 对应验收标准：设置面板列出内置 resource pack。

2. Rust Core 持久化命令
   - 在 `CreatureState` 增加 active pet setter。
   - 在 `IanRuntime` 增加 `save_active_pet`。
   - 在 Tauri commands 和 invoke handler 注册 `save_active_pet`。
   - 对未知 id 返回错误。
   - 对应验收标准：保存 state、拒绝未知 id。

3. Tauri bridge
   - 在 `tauriBridge.ts` 增加 `saveActivePet`。
   - 浏览器 fallback 同步更新 `active_pet_id` / `active_resource_pack`。
   - 对应验收标准：前端能调用保存命令。

4. React 启动加载和切换流程
   - App 初始读取 settings 后加载 `state.active_resource_pack`。
   - 增加 active pet 本地状态。
   - 增加切换 handler：先 `loadPetResourcePack(nextId)`，加载成功后更新资源包并调用 `saveActivePet(nextId)`；失败则保留当前资源包。
   - 对应验收标准：启动使用持久化值、加载失败不保存。

5. 设置面板 UI
   - SettingsPanel 增加宠物选择器 props 和 UI。
   - 放在生活分组中，避免高级设置里隐藏核心外观。
   - 对应验收标准：用户可见并可切换。

6. 验证与记录
   - 跑 Rust / 前端目标测试、类型检查。
   - 尝试真实 Tauri 桌面验收并记录。

## 预计改动文件

- `apps/desktop/src/resources/resourceLoader.ts`
- `apps/desktop/src/App.tsx`
- `apps/desktop/src/lib/tauriBridge.ts`
- `apps/desktop/src/renderer/SettingsPanel.tsx`
- `apps/desktop/src/renderer/SettingsPanel.view.test.tsx`
- `apps/desktop/src-tauri/src/core/creature_state.rs`
- `apps/desktop/src-tauri/src/app/runtime.rs`
- `apps/desktop/src-tauri/src/desktop/commands.rs`
- `apps/desktop/src-tauri/src/lib.rs`
- `apps/desktop/src-tauri/src/domain/behavior/behavior_engine.rs` 或 runtime tests 位置（仅测试需要时）

## 接口影响

- 新增 Tauri command `save_active_pet`。
- 无 `IanEvent` / `IanAction` 协议变化。
- `IanState` 字段保持不变。

## 兼容性

- 老配置中的 `active_pet` / `active_resource_pack` 继续生效。
- 默认值仍由 Rust `IanState::default()` 决定。
- 前端硬编码默认仅作为 settings 加载失败时 fallback。

## 风险

- 已有 resource pack sprite 尺寸和动画命名不完全一致，切换后可能视觉大小不同；本轮不统一资源包美术。
- 桌面端热加载 resource pack 若失败，需要回退当前资源包并避免写入 Rust state。

## 回滚

- 删除设置面板宠物选择器。
- App 恢复硬编码默认 resource pack 加载。
- 保留 Rust state 字段不变。

