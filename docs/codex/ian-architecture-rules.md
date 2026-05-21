# Ian Architecture Rules

This document captures stable architecture rules for Ian. These rules should change slowly.

For current product scope, read `ian-current-stage.md`.

## Product Identity

Ian is a local-first desktop digital creature.

Ian is not:

- an AI assistant
- a Copilot clone
- a productivity workflow bot
- a chat UI with a mascot

All technical capability must serve:

- life-like presence
- companionship
- emotional comfort
- privacy
- user control

If a feature is powerful but makes Ian feel like a tool instead of a creature, do not prioritize it.

## Dual-Track Strategy

Use two tracks at the same time:

- Product features evolve in small, testable slices.
- Technical architecture is designed for long-term extension from the start.

Code cost is not the main constraint. Avoid future rework by establishing the right boundaries early.

Early modules may be skeleton, minimal, no-op, or feature-gated. The boundary should still be correct.

## Everything Is an Event

All outside-world input must become `IanEvent`.

All creature output must become `IanAction`.

The frontend-visible state must be represented as `IanState`.

Rust is the protocol source of truth. TypeScript protocol types should be generated from Rust types.

Do not hand-maintain parallel Rust and TypeScript protocol definitions.

## Runtime Responsibility

Rust Core is Ian's brain.

Rust Core owns:

- event bus
- mood engine
- bond engine
- behavior engine
- reminder engine
- policy engine
- dialogue orchestration
- scheduler / tick
- state manager
- storage repositories
- security / permission gate

React is Ian's body.

React owns:

- sprite rendering
- animation playback
- bubble UI
- input widgets
- settings surfaces
- visual feedback

React must execute `IanAction`. It must not decide Ian's core behavior, long-term state, mood, bond, or policy.

## Adapter Rules

Adapters are Ian's perception layer.

Adapters:

- collect external signals
- declare source, sensitivity, and permission requirements
- convert signals to `IanEvent`
- pass through Security Gate before affecting Runtime

Adapters must not:

- directly control animations
- directly mutate creature state
- bypass rate limits
- read sensitive sources by default

High-sensitivity adapters are off by default, including:

- clipboard
- screen text / OCR
- code content
- private chat content
- current file content
- terminal full output

## Storage Rules

Ian is local-first.

Use `config.toml` for user-editable configuration and SQLite for long-term local state.

Storage should be designed around:

- migrations
- normalized core entities
- append-only interaction / adapter event logs where useful
- explicit repositories
- JSON only for extension payloads, not as a replacement for core fields

P0 can persist only basic config, position, and interaction events, but the storage skeleton should support later mood, bond, reminders, memory, adapters, and social records.

## Resource Pack Rules

Ian's visual identity should be resource-pack based from the beginning.

The default pet pack should live under:

```txt
public/resources/pets/ian-alpaca/
```

Expected resource pack files:

- `pet.json`
- `animations.json`
- `expressions.json`
- sprite image
- optional sounds directory

Do not hardcode Ian as a single fixed image or emoji in application logic.

## Dialogue Rules

LLM support is a language ability, not Ian's identity.

Dialogue must obey:

- Ian identity
- mood context
- bond context
- response policy
- safety policy
- output length limits

Ian should not say "as an AI assistant".

Default responses should be short, characterful, and bubble-friendly.

Demo Dialogue must keep Ian usable without API keys or network access.

OpenAI-compatible BYOM should use provider boundaries and secure key storage, but it must not make the product feel like a ChatGPT desktop skin.

## Security Rules

All external input passes through Security Gate.

Security Gate checks:

- source legitimacy
- permission
- rate limit
- payload size
- sensitivity
- sanitization requirements

Sanitize external text before rendering it on the desktop, especially future Feishu, Pet Visit, plugin, and LLM output.

Default allowed:

- time
- Ian window mouse events
- local demo dialogue
- local configuration

Requires explicit authorization:

- global keyboard rhythm
- Git project information
- active app / window state
- IDE state
- Feishu

Default forbidden:

- code body
- clipboard
- private chat content
- screen OCR
- arbitrary file content

