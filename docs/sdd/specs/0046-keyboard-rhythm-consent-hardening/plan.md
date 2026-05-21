# 实现计划: Keyboard Rhythm Consent Hardening

## 对应规格

`docs/sdd/specs/0046-keyboard-rhythm-consent-hardening/spec.md`

## 实现步骤

1. 为 keyboard adapter 默认关闭、开启和关闭清理写失败测试。
2. 加固 sanitizer，拒绝 key/text/shortcut 字段。
3. 确认 Developer Rhythm 总开关不自动开启 keyboard rhythm。
4. 设置面增加短说明。
5. 做 source scan 和 opt-in / opt-out smoke。

## 预计改动文件

- `apps/desktop/src-tauri/src/adapters/keyboard_rhythm_adapter.rs`
- `apps/desktop/src-tauri/src/security/sanitizer.rs`
- `apps/desktop/src-tauri/src/security/permission.rs`
- `apps/desktop/src/renderer/SettingsPanel.tsx`
- `docs/sdd/specs/0046-keyboard-rhythm-consent-hardening/*`

## 接口 / 兼容性

保持 `keyboard.rhythm` payload 结构不变。旧授权配置缺失时默认关闭。

## 验证命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml adapters
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml security
npm run desktop:test
```

## 风险和回滚

键盘节奏隐私风险高。回滚方式是保留设置项但禁用 adapter 输出。
