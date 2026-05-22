# 178 · 验证记录

## 2026-05-22

### 本轮覆盖

- Memory：低敏 tags 候选、确认后查询、删除、过期清理、清空。
- Mood / Bond / Life：bond internal milestone、mood 动画权重、每日问候 once-per-day、quiet 抑制、久别返回无责备、personality 频率权重、亲近文案禁用词。
- Social / Pet Visit：Feishu skeleton 默认关闭且不联网、远端消息白名单和清洗、visit invitation 过期、未知动作拒绝、本地资源 allowlist、group broadcast 长度限制、rate limit / block、readiness gate。
- Platform：plugin manifest 校验、未知权限拒绝、插件授权撤销、resource import 拒绝脚本和远端引用、sound 默认关闭和音量上限、多屏安全区域策略。
- Docs：补充 multi-Ian、storage migration v2、no default telemetry、contributor onboarding、v1 platform readiness map。

### RED 验证

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml memory_candidates_require_confirmation_and_keep_only_low_sensitive_tags`
  - 结果：失败，缺少 MemoryRepository、Social/Plugin/Resource skeleton 类型；确认新增测试能捕获未实现能力。

### 已执行命令

- `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 结果：通过。
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:test`
  - 结果：通过，12 个测试文件，87 个测试。
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:typecheck`
  - 结果：通过。
- `PATH=/opt/homebrew/bin:$PATH npm run desktop:build`
  - 结果：通过，Vite production build 完成。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 结果：通过，146 个 Rust 测试。
- `rg -n "analytics|telemetry|sentry|segment|posthog|amplitude|crashlytics|http://|https://" apps/desktop/src apps/desktop/src-tauri/src docs/codex/no-default-telemetry-policy.md docs/codex/v1-platform-readiness-map.md`
  - 结果：未发现默认遥测调用；命中项为 no-default-telemetry 文档、远端资源拒绝测试/检查、BYOM provider 默认 base URL 和 CSS class 名。

### 桌面验收

- 启动 dev server：`PATH=/opt/homebrew/bin:$PATH npm run dev --workspace @ian/desktop -- --host 127.0.0.1 --port 5174`
  - 结果：Vite 在 `http://127.0.0.1:5174/` ready。
- 启动真实 Tauri 桌面壳：`PATH=/opt/homebrew/bin:$PATH npm run tauri --workspace @ian/desktop -- dev --no-dev-server-wait --config '{"build":{"beforeDevCommand":"","devUrl":"http://127.0.0.1:5174"}}'`
  - 结果：Rust dev build 完成并运行 `target/debug/ian_desktop`，启动期无错误；本轮启动的 Tauri 和 dev server 已停止。

### 安全 / 隐私验证

- 默认关闭：Feishu、Pet Visit、Plugin、Sound、跨屏漫游均默认关闭或未授权拒绝。
- 低敏保存：memory 只允许 tags，不允许 `text:`、路径和 HTML；visit records 只保存摘要。
- 本地优先：远端 pet resource、resource import 中的 URL、脚本资源和未知 plugin permission 被拒绝。
- 无遥测：新增策略文档明确默认无 endpoint 和自动上报，代码搜索未发现默认遥测调用。

### 剩余风险

- 本轮没有启用完整用户可见社交、插件、导入、声音或多实例能力；这些只作为后续 SDD 的安全骨架。
- 记忆候选 UI 目前是最小审核入口文案，不展示真实数据库列表。
- 桌面 smoke 覆盖启动期和真实壳运行，不等同于完整人工交互验收。
