# 记忆低敏标签体系

Ian 的记忆候选只能保存短标签，不保存聊天全文、代码、路径、剪贴板、私聊或屏幕文字。

## 允许标签

| 前缀 | 用途 | 示例 |
| --- | --- | --- |
| `pref:` | 用户明确表达的陪伴偏好 | `pref:quiet` |
| `topic:` | 低敏日常主题 | `topic:water` |
| `routine:` | 用户明确允许的轻量日常习惯 | `routine:stretch` |
| `moment:` | 粗粒度时段或场景 | `moment:morning` |
| `place:` | 非精确、非地址化位置偏好 | `place:window` |

标签值只能使用小写 ASCII、数字、`_` 和 `-`，单个值不超过 48 字符。

## 禁止标签

- `text:` 或任何原始自然语言正文。
- `code:`、函数片段、stack trace 或源码内容。
- 文件路径、URL、用户主目录、项目路径。
- HTML / script / 富文本片段。
- 私聊内容、密码、token、API key、个人身份信息。
- 未列入允许表的任意前缀。

## 使用边界

- candidate 只能进入审核区。
- confirmed 后才可进入短对话上下文。
- 对话中最多使用少量 confirmed tags，不能让记忆主导回复。
- 删除或清空候选必须是用户明确动作。

