# 验证记录: Dialogue Policy and BYOM Boundary

## Spec

`docs/sdd/specs/0013-dialogue-policy-byom-boundary/spec.md`

## 状态

已验证，待用户验收。

## 检查项

| 检查项 | 命令 / 方法 | 结果 | 备注 |
| --- | --- | --- | --- |
| Dialogue tests | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml dialogue` | 通过 | 6 tests passed。 |
| Security tests | `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml security` | 通过 | 4 tests passed。 |
| Frontend tests | `npm run desktop:test` / `npm run desktop:typecheck` | 通过 | 6 files / 16 tests passed；Demo provider 仍是默认路径。 |

## 验收标准结果

- [x] DialoguePolicy 对长度、身份措辞和空输出有测试。
- [x] Demo Provider 输出始终经过 DialoguePolicy。
- [x] BYOM config 边界存在，但默认不发起网络请求。
- [x] 不记录 API Key 或完整敏感输入日志。
- [x] 未配置 BYOM 时 Demo Dialogue 正常工作。

## 失败或缺口

真实 BYOM 调用仍是 no-op 边界，不在本批开启。

## 后续

如后续开启真实 provider，需要单独 SDD 覆盖密钥存储和网络权限。
