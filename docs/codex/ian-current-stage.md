# Ian Current Stage

This document defines the current execution stage. It should be updated as Ian moves from P0 to v0.1.x, v0.2, and later phases.

Current stage:

```txt
P0 / MVP + v0.1 Architecture Baseline
```

## Goal

Validate whether Ian feels alive enough for a user to want it living on the desktop.

At the same time, establish the long-term engineering skeleton so future iterations do not require repeated architecture rewrites.

## P0 Product Scope

Build the smallest user-visible creature experience:

- transparent always-on-top desktop window
- Ian appears near the bottom-right of the desktop
- basic sprite / placeholder creature rendering
- idle, walk, happy, and run animations
- click opens a speech bubble
- Demo Dialogue returns short characterful replies
- double-click triggers a run-around surprise
- basic drag / position persistence

P0 is about "it is alive", not feature completeness.

## v0.1 Architecture Baseline Scope

Build the long-term skeleton early:

- Tauri v2 desktop app
- React + Vite + TypeScript frontend
- Rust Core Runtime inside `src-tauri`
- Rust-source protocol types
- TypeScript protocol generation
- `IanEvent`
- `IanAction`
- `IanState`
- Resource Pack directory and manifest files
- Adapter trait
- Time / Mouse / Dialogue minimal adapters
- Behavior policy boundary
- Dialogue provider boundary
- SQLite + config skeleton
- migration skeleton
- repository skeleton
- Security Gate / Permission / Sanitizer / Rate Limiter skeleton

Skeleton, minimal, and no-op implementations are acceptable when the product behavior is not ready yet.

## Not in P0

Do not implement these as user-visible P0 features:

- GitAdapter
- KeyboardRhythmAdapter
- build / test / CI events
- global keyboard monitoring
- active window awareness
- code reading
- terminal output reading
- full Mood System
- full Bond System
- long-term memory
- proactive reminder system
- Feishu
- Pet Visit
- plugin system
- multi-pet / multi-Ian support
- complex settings UI

Architecture hooks for these are allowed when they keep boundaries clean.

## BYOM / LLM Position

Demo Dialogue is required for P0.

OpenAI-compatible BYOM is optional for P0:

- Provider boundaries should exist in the architecture.
- Secure key storage should be planned.
- User-visible BYOM can move to v0.1.x if it risks making P0 feel like a chat product.

Default recommendation: do not make BYOM the core P0 selling point.

## First Implementation Slice

Start with:

1. Monorepo / app skeleton
2. Tauri desktop shell
3. React rendering layer
4. Rust protocol and runtime skeleton
5. Resource Pack placeholder
6. click -> `IanEvent` -> Rust Core -> `IanAction` -> bubble / animation
7. double-click run-around behavior
8. basic position persistence

## Definition of Done for P0

P0 is acceptable when:

- the app launches on macOS
- a transparent always-on-top Ian window appears
- Ian renders from the resource pack or placeholder pack
- Ian can idle and play at least one active animation
- clicking Ian produces a speech bubble
- the speech bubble uses Demo Dialogue
- double-click triggers a run-around surprise
- behavior decisions pass through Rust Core
- React renders actions instead of owning behavior decisions
- basic config or position survives restart

## Next Stage

After P0 feels alive, move into v0.1.x:

- gentle drink / rest reminders
- more animations and expressions
- Moment System / small creature moments for life-like surprise
- simple settings
- simple Mood State
- simple Bond State
- stronger SQLite persistence
- optional BYOM UI

Moment System is a v0.1.x Life Feel direction, not a P0 scope expansion. Its detailed architecture lives in `docs/codex/ian-moment-system.md`. Moments should compose existing local events, state, animations, bubbles, movement, cooldowns, and privacy boundaries into short, low-frequency scenes.

Developer Rhythm belongs in v0.2:

- Git events
- build / test events
- keyboard rhythm
- basic developer-context reactions
