# Decisions: Puppy Resource Pack + Micro Life Upgrade

| Date | Decision | Reason | Consequence |
| --- | --- | --- | --- |
| 2026-05-21 | 0082 合并 puppy 视觉、微动作、短期日常性格为一个 product-feel slice。 | 三者共同解决“灵动性不足”，拆开实现会让体验不连续。 | 实现必须分阶段验证，避免范围扩张。 |
| 2026-05-21 | 短期日常性格只放在 Rust Core 内存态，不持久化。 | 用户要的是 5-10 分钟延续感，不是完整 Mood / Bond 或长期记忆。 | 重启后状态丢失，隐私和范围更稳。 |
| 2026-05-21 | 优先复用现有动画语义，只有确需时才新增 animation name。 | 降低协议、资源包和 fallback 改动面。 | Puppy 可以先通过更多 frame 序列表现灵动。 |
| 2026-05-21 | 新安装默认可显示 `ian-puppy`，但已有配置不强制迁移。 | 用户控制优先；资源升级不应覆盖用户选择。 | 需要测试覆盖 config 优先级。 |
| 2026-05-21 | 0082 不新增 `settle` / `stretch` / `look` 协议动画名。 | 现有 `happy`、`rest`、`idle`、`effect.play` 足够表达第一版微动作和 settle。 | 旧资源包无需同步补帧，`ian-puppy` 通过更丰富 frame 序列承担视觉升级。 |
| 2026-05-21 | idle 微动作第一版使用 `effect.play tail_wag`。 | 这能在不打断当前 idle animation 的前提下提供低打扰生命迹象。 | 前端 CSS 需要支持 `tail_wag`，reduced motion 下沿用隐藏效果规则。 |
| 2026-05-21 | Puppy 视觉验收反馈后改为 `rounded-puppy-v2`，不继续沿用尖耳、硬折线的第一版。 | 第一版图标感太强，不像柔软桌面宠物；用户明确反馈“狗好丑”。 | SVG sprite 改成圆脸、短腿、小圆耳、柔和尾巴，并用 resource test 锁定版本标记。 |
| 2026-05-21 | 高能动作改成“慢一些 + 轻微确定性变化”，不使用真随机。 | 用户希望不要动得非常快；确定性变化能带来不机械的节奏，同时测试稳定、可复现。 | `movement.move_to` 每步从 180/280/420ms 调整到约 380/520/740ms，并按链路步数做小幅变化；run/zoomies sprite FPS 与 CSS loop 同步降速。 |
