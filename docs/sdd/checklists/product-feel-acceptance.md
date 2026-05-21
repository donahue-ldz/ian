# Product Feel Acceptance Checklist

## Scope

This checklist validates whether Ian still feels like a local desktop creature after product-feel and playful-expression changes.

## Commands

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npm run desktop:test
npm run desktop:typecheck
npm run desktop:build
```

## First 30 Seconds

- Ian appears as a small creature surface, not a chat app or notification center.
- The settings entry is present but visually quiet.
- Default visible UI does not expose developer rhythm, Git, build, keyboard, or app-presence concepts.
- A click opens a short creature-style bubble.

## Five Minute Companionship

- Idle, rest, and sleep states remain visually distinct.
- Ian does not blank, freeze, spam bubbles, or move constantly.
- Quiet mode and reduced motion lower visual intensity.
- Sleep and bubbles do not visually fight for space.

## Core Interaction

- Single click shows a short affectionate reaction.
- Double click still performs short run-around.
- High playful energy can trigger zoomies through Rust Core.
- Drag start and drag end have distinct gentle reactions.
- Bubble input can open, submit, and close without being overwritten by spontaneous speech.
- Settings can be opened, changed, and closed.

## Privacy And Developer Rhythm

- Developer rhythm remains behind the optional settings disclosure.
- Privacy copy states what is read and what is not read.
- Developer sources are disabled by default.
- Playful diagnostics contain only reason, result, key, and timestamp fields.

## Playful Safety

- `playful_energy = off` prevents spontaneous zoomies.
- Quiet mode, quiet hours, drag, and bubble input block spontaneous high-energy behavior.
- Zoomies use bounded waypoints and return to idle or anchor.
- Cooldown prevents stacked zoomies without blocking ordinary click feedback.

## Screenshots To Record

- Default first screen.
- Bubble feedback.
- Settings privacy/developer disclosure.
- Idle/rest/sleep.
- Playful energy setting.
- Zoomies or visual-effect state.
