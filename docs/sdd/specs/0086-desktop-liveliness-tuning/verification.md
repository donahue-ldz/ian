# 0086 验证记录

## TDD 红灯

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml pointer_chase -- --nocapture`
  - 初始失败：追鼠标成功路径只有 1 段 `movement.move_to`，不满足多段追逐验收。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml micro_motion -- --nocapture`
  - 初始失败：普通模式 15 秒 idle tick 未产生微动作，不满足更容易观察的节奏要求。

## 针对性验证

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml pointer_chase -- --nocapture`
  - 通过：4 passed。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml micro_motion -- --nocapture`
  - 通过：2 passed。

## 全量验证

- `npm run desktop:typecheck`
  - 通过。
- `npm run desktop:test`
  - 通过：12 files passed，74 tests passed。
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - 通过：110 tests passed。
- `git diff --check`
  - 通过。

## 真实桌面端验收

- 启动方式：`npm run desktop:tauri -- dev --host 127.0.0.1`。
- 桌面进程：`target/debug/ian_desktop 127.0.0.1 --no-default-features --color always --`。
- 窗口定位：`ian_desktop` / `Ian`，初始 bounds 为 `X=1208, Y=214, Width=260, Height=340`。
- 验收动作：在真实桌面截图中定位 Ian 可见身体中心，点击 Ian 后将鼠标移开。
- 验收结果：Ian 窗口移动到 `X=800, Y=385`，截图中可见追逐后的跑动效果线，说明桌面点击 + 移开鼠标触发追逐链路已生效。
- 截图记录：
  - `/tmp/ian-0086-running.png`
  - `/tmp/ian-0086-after-center-click-leave.png`

## 额外发现

- 第一次用估算坐标点击时落到了 Chrome 内容区，没有触发 Ian；原因是坐标没有命中宠物可见身体中心。后续用截图颜色范围定位 Ian 身体中心后，点击和追逐均可触发。
- 桌面壳重编译时发现 `ConfigFile -> IanState` 少了 `screen_bounds` / `last_user_interaction_ms` 默认值，已做最小兼容补齐，否则桌面端无法启动完成验收。
