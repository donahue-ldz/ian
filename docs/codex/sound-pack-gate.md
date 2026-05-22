# Sound Pack Gate

Sound packs are default-off and not a P0 product feature.

The architecture may keep a local-only policy skeleton, but no sound assets are shipped and Ian must remain visually usable without audio. Any future sound surface needs do-not-disturb handling, a volume cap, local asset validation, and an explicit user setting before playback.

Required gates before productization:

- default-off playback with clear user opt-in.
- do-not-disturb suppresses playback.
- volume cap prevents loud surprise audio.
- local-only audio assets with no remote references.
- no sound assets are shipped until the product loop is approved.
