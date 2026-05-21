# Spec: Pet Resource Pack Switcher

## 问题 / 目标

当前仓库中已有多个 resource pack（`ian-puppy`、`ian-kitten`、`ian-alpaca`、`ian-adventurer`），但前端启动时硬编码加载默认资源包，用户无法在应用内切换。

本 SDD 目标是在设置面板中补一个宠物外观切换入口，让用户切换当前 Ian 的 resource pack，并持久化到 Rust Core 的 `IanState.active_pet_id` / `IanState.active_resource_pack`。

## 当前产品阶段

P0 / MVP + v0.1 Architecture Baseline。

本功能是 resource pack skeleton 的最小可见闭环，不扩展为多只宠物系统。

## 产品范围

- 设置面板显示一个“宠物”选择器。
- 可选项来自当前已打包的 resource pack：
  - `ian-adventurer`
  - `ian-puppy`
  - `ian-kitten`
  - `ian-alpaca`
- 选择后当前桌面 Ian 立即加载对应 resource pack。
- 选择结果持久化到本地 `config.toml`。
- 下次启动时优先加载持久化的 `active_resource_pack`，而不是硬编码默认值。

## 明确不做

- 不做多只 Ian 同时存在。
- 不做宠物身份、记忆、心情、羁绊按宠物拆分。
- 不做资源包下载、商城、导入、删除或远程更新。
- 不做复杂预览画廊。
- 不迁移历史配置默认宠物；缺省仍由 Rust `IanState::default()` 控制。

## 用户体验

- 用户打开设置。
- 在“生活”或相近的设置分组中看到“宠物”选择器。
- 选择不同宠物后，屏幕上的 Ian 外观立即切换。
- 如果资源包加载失败，前端回退到原来的已加载资源包，不保存失败选择。

## 架构约束

- React 只负责设置 UI、加载 resource pack、执行视觉切换。
- Rust Core / Storage 负责持久化当前 active pet/resource pack。
- 不通过 `IanEvent` 表达该设置变更；现有设置项也通过 Tauri command 持久化，本功能沿用设置命令模式。
- TypeScript 不新增协议字段；复用已有 `IanState.active_pet_id` / `IanState.active_resource_pack`。

## 数据 / 协议变化

- 新增 Tauri command：`save_active_pet(active_pet_id: String)`。
- Runtime 将 `active_pet_id` 和 `active_resource_pack` 同步设置为同一个已知 resource pack id。
- 不修改 `IanAction` / `IanEvent`。

## 隐私与安全边界

- 不读取文件内容、窗口内容、剪贴板或网络。
- 只保存一个本地 resource pack id。
- Rust command 拒绝空 id 和未知 id，避免持久化任意路径式资源包标识。

## 验收标准

- [ ] 设置面板打开时能看到“宠物”选择器，并列出当前内置 resource pack。
- [ ] 用户选择一个宠物后，React 调用保存命令，并在保存成功后加载对应 resource pack。
- [ ] 保存成功后 `IanState.active_pet_id` 和 `IanState.active_resource_pack` 都更新为选中的 id。
- [ ] 下次启动 / 加载设置时，App 按 `state.active_resource_pack` 加载 resource pack。
- [ ] 未知宠物 id 会被 Rust command 拒绝，不会写入 state。
- [ ] resource pack 加载失败时，前端不保存失败选择，并保留当前已加载资源包。

## 验证方式

- Rust 单元测试：
  - 保存有效宠物 id 更新 state。
  - 保存未知宠物 id 返回错误。
- 前端测试：
  - SettingsPanel 渲染宠物选择器。
  - resource pack 切换流程先加载再保存，加载失败不保存。
- 类型检查：
  - `tsc --noEmit`
- 桌面端验收：
  - 启动真实 Tauri 桌面壳。
  - 打开设置，切换宠物。
  - 观察桌面 Ian 外观立即变化。
  - 关闭重启后确认仍加载上次选择。

