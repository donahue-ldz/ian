# 实现计划: Active App Presence Boundary

## Spec

`docs/sdd/specs/0020-active-app-presence-boundary/spec.md`

## 状态

已实现，待验收。

## 概要

建立活动应用感知的安全边界，只允许粗粒度类别，不读取窗口内容。

## 步骤

1. 定义 presence category 和权限项。
2. 编写 Security Gate 默认拒绝测试。
3. 实现 adapter skeleton / mocked source。
4. 将 category 接入提醒或开发节奏 policy。
5. 做 payload/source scan。

## 预计文件改动

- `apps/desktop/src-tauri/src/adapters/*`
- `apps/desktop/src-tauri/src/security/*`
- `apps/desktop/src-tauri/src/domain/behavior/*`
- `apps/desktop/src-tauri/src/domain/reminder/*`

## 接口与边界

只允许粗粒度 app category；禁止窗口标题、URL、文件名和屏幕文字。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml adapters
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml security
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml behavior
rg -n "window title|title|url|ocr|screen|document" apps/desktop/src-tauri/src
```

## 风险

- 活动应用信息容易越界；0020 只做 category boundary，不做真实窗口内容感知。

## 回滚说明

保持 adapter disabled 或删除 presence event handling。
