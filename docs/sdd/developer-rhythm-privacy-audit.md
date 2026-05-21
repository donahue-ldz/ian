# Developer Rhythm Privacy Audit

## 审计目标

Developer Rhythm 只能在用户显式授权后消费低敏开发节奏摘要。它不得读取、保存或展示代码正文、diff、终端全文、按键内容、窗口标题、URL、屏幕 OCR 或剪贴板内容。

## 允许字段

- Git: `workspace_id`、`branch`、`dirty`、`short_commit`
- Build/test: `workspace_id`、`tool`、`status`、`duration_ms`、`tests_total`、`tests_failed`、`error_kind`
- Keyboard rhythm: `window_ms`、`intensity`、`count`
- Active app presence: `category`、`confidence`、`app_id`

## 必查项

- [ ] Developer Rhythm capability 默认关闭。
- [ ] Git / build-test 事件需要 capability 开启且 workspace 已绑定。
- [ ] Keyboard rhythm 不随 Developer Rhythm 总入口自动开启。
- [ ] 事件反序列化拒绝未知敏感字段。
- [ ] Sanitizer 限制低敏字段长度和格式。
- [ ] diagnostics / life events 只记录事件类型、action 类型和策略原因。
- [ ] source scan 覆盖敏感词：`git diff`、`stdout`、`stderr`、`key_code`、`keypress`、`keydown`、`window_title`、`document.title`、`clipboard`、`screenshot`、`OCR`、`read_to_string`。

## 审计命令

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml security
rg -n "git diff|stdout|stderr|key_code|keypress|keydown|window_title|window title|document.title|clipboard|screenshot|OCR|read_to_string" apps/desktop/src apps/desktop/src-tauri/src -S
```

## 当前说明

Source scan 命中测试用例中的敏感字段字符串是预期结果，用于证明反序列化和 sanitizer 会拒绝这些 payload。生产路径命中必须逐项审查。
