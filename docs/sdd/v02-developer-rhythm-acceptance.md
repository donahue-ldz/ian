# v0.2 Developer Rhythm Acceptance

## 通过条件

- [ ] Developer Rhythm 默认关闭，用户可逐项开启 Git、build/test、keyboard rhythm 和 active app presence。
- [ ] Git / build-test 在 workspace 未绑定时不产生用户可见开发者反应。
- [ ] 授权且绑定 workspace 后，Git / build-test mock event 可以产生低频、短句、Ian 风格反应。
- [ ] Keyboard rhythm payload 只包含时间窗口、强度和计数。
- [ ] Active app presence 只消费粗粒度 category，不读取窗口标题、URL 或屏幕文字。
- [ ] Developer Rhythm snooze、quiet hours、meeting / presentation / focus category 会降级开发者反应。
- [ ] Privacy audit checklist 通过。

## 明确排除

- 代码分析建议
- diff / 源码正文读取
- 终端 stdout / stderr 全文读取
- 自动运行测试或 Git 写操作
- Feishu、Pet Visit、插件系统
- 云同步或远程遥测

## 自动验收命令

```bash
npm run desktop:acceptance
rg -n "git diff|stdout|stderr|key_code|keypress|keydown|window_title|window title|document.title|clipboard|screenshot|OCR|read_to_string" apps/desktop/src apps/desktop/src-tauri/src -S
```

## 人工体验验收

打开设置，检查“开发者节奏”分组、工作区绑定、逐项能力开关和暂停选项。开启工作区和 Git / build-test 后，用 mock event 验证 Ian 只出现短句和轻量动画，不展示代码、日志或窗口内容。
