# AGENTS.md

This file is the root instruction entrypoint for Codex and other coding agents working in this repository.

Keep this file short and authoritative. Detailed rules live in `docs/codex/`, but the constraints below must remain visible here so they are loaded reliably.

## Required Reading

Before implementing, modifying, or scaffolding Ian code, read these files:

1. `docs/codex/engineering-defaults.md`
2. `docs/codex/ian-architecture-rules.md`
3. `docs/codex/ian-current-stage.md`
4. `docs/codex/sdd-workflow.md`

Use `ian.md` as the full product and architecture source of truth when a detail is not covered in the Codex docs.

If these documents conflict, follow this priority:

1. Direct user instruction in the current conversation
2. This `AGENTS.md`
3. `docs/codex/sdd-workflow.md`
4. `docs/codex/ian-current-stage.md`
5. `docs/codex/ian-architecture-rules.md`
6. `docs/codex/engineering-defaults.md`
7. `ian.md`

## Non-Negotiable Ian Constraints

Ian is not an AI assistant, Copilot clone, productivity bot, or ChatGPT desktop skin.

Ian is a local-first desktop digital creature. Every product and technical choice should preserve:

- life-like presence
- companionship
- emotional comfort
- user control
- privacy by default

## Current Product Strategy

Use the dual-track strategy:

- Product features evolve in small visible slices.
- Technical architecture is designed for the long-term ideal from day one.

Current execution target:

- P0 / MVP validates whether Ian feels alive enough to live on the desktop.
- v0.1 Architecture Baseline establishes the long-term engineering skeleton.

Do not turn P0 into a full product.

## SDD Workflow

Use Spec-Driven Development for non-trivial work.

The user is not expected to write SDD documents manually. The agent must generate and maintain them before implementation.

For feature work, architecture work, multi-file changes, data model changes, behavior changes, or scaffolding:

1. Create or update `docs/sdd/specs/<id>-<slug>/spec.md`.
2. Create or update `docs/sdd/specs/<id>-<slug>/plan.md`.
3. Get user approval before implementation unless the user explicitly grants approval in the same turn.
4. Implement only what the approved spec and plan allow.
5. Record scope or design changes in `decisions.md`.
6. Record verification commands and results in `verification.md`.

For any Ian desktop-visible capability, browser verification is not sufficient. Verify the behavior in the real Tauri desktop shell before accepting the SDD. If desktop verification cannot run, record the reason, substitute checks, and remaining risk in `verification.md`.

Small typo fixes, formatting-only edits, or simple read-only analysis do not need a full SDD packet.

## P0 Scope

P0 should stay small:

- transparent always-on-top desktop pet
- basic sprite animation
- click bubble
- short Demo Dialogue reply
- double-click run-around surprise
- basic position / config persistence

P0 must not implement:

- Git / build / test integration
- global keyboard monitoring
- full Mood System
- full Bond System
- long-term memory
- proactive reminders
- Feishu
- Pet Visit
- plugin system
- window-aware behavior

These can have architecture skeletons, but should not become user-visible product features in P0.

## Architecture Rules

Everything is an Event.

- All external input enters Rust Core as `IanEvent`.
- All behavior output leaves Rust Core as `IanAction`.
- Shared current state is represented as `IanState`.
- Rust types are the protocol source of truth.
- TypeScript protocol types should be generated from Rust types.

Responsibility split:

- Rust Core owns creature behavior, mood, bond, policy, dialogue orchestration, scheduling, storage, and security gates.
- React owns rendering, animation playback, bubble UI, user input widgets, and settings surfaces.
- React must execute `IanAction`; it must not become Ian's behavior brain.
- Adapters collect outside-world signals and convert them to `IanEvent`; adapters must not directly control animation or persistent state.

Build the v0.1 skeleton early:

- Tauri desktop shell
- Rust Core Runtime
- protocol layer
- Resource Pack structure
- Adapter trait
- Storage / migration skeleton
- Security / permission skeleton
- Dialogue provider boundary
- Behavior policy boundary

Skeleton / no-op / minimal implementations are acceptable in early stages.

## Engineering Defaults

Work as a pragmatic senior engineer:

- Understand first, then design, then implement.
- Keep changes scoped to the task.
- Prefer existing project patterns and tools.
- Do not introduce dependencies or abstractions without clear payoff.
- Do not overwrite user changes.
- Do not use destructive git commands unless explicitly requested.
- Do not hardcode secrets or log sensitive data.
- Verify relevant behavior before claiming completion.

## Delivery Standard

Unless the user asks only for a draft, deliver work that is:

- runnable
- verifiable
- clearly named
- modular
- consistent with the repository
- free of unrelated refactors
- explicit about remaining risks
