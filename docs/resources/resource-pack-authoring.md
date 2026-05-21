# Resource Pack Authoring

## 目标

Ian resource pack 是 React 渲染层的视觉来源。Rust Core 只输出语义动画名，例如 `idle`、`walk`、`happy`、`run`、`sleep`，资源包负责把语义映射到帧和表情。

## 文件结构

```txt
public/resources/pets/<pack-id>/
  pet.json
  animations.json
  expressions.json
  sprite.svg
  sounds/
```

Bundled examples currently include `ian-alpaca`, `ian-kitten`, and `ian-puppy`.
`ian-puppy` is the preferred example for high-liveliness packs because its idle
sequence contains multiple frames for breathing, blinking, and tail motion while
still using the same semantic animation contract.

## `pet.json`

必填字段：

- `id`
- `name`
- `version`
- `species`
- `defaultPersonality`
- `sprite`
- `animations`
- `expressions`
- `sounds`
- `capabilities`

`animations` 和 `expressions` 必须指向同目录下的 JSON 文件。缺少必填字段会抛出包含具体字段名的 `ResourcePackError`，例如 `pet.animations`。

## `animations.json`

必填：

- `meta.frameWidth`
- `meta.frameHeight`
- `meta.scale`
- `animations.idle`

建议提供：

- `animations.walk`
- `animations.happy`
- `animations.run`
- `animations.sleep`
- `animations.rest`
- `animations.zoomies`

未知动画名会 fallback 到 `idle`。缺失 `idle` 是硬错误，因为渲染层需要稳定默认状态。

微动作可以优先复用现有语义动画和 `effect.play`，不要为了每个小动作新增协议名。旧资源包缺少更细帧时应 fallback 到 `idle`、`happy` 或 `run`。

## `expressions.json`

建议提供：

- `expressions.idle`
- `expressions.happy`

未知表情会 fallback 到 `expressions.idle`，如果 `idle` 也缺失，则 fallback 到 `{ "overlay": null }`。

## 隐私与安全

资源包校验只读取 Ian 自身资源目录，不扫描任意用户文件，不联网下载资源，不执行资源包内脚本。
