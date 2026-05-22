# Life Rhythm Acceptance

This checklist anchors SDDs 0203-0210 without expanding P0 beyond a small desktop creature loop.

## Sleep And Wake

- Long idle ticks may enter `rest` or `sleep`.
- Direct user interaction, find Ian, or a wake click returns Ian to a visible awake state.
- Wake copy stays short and non-demanding.

## Daily Greeting

- Ian may greet once per local day after the first local day index.
- Quiet mode, do-not-disturb, and quiet hours suppress proactive greeting.
- Greeting copy is short and does not ask the user to do work.

## Absence Return

- Ian may react once after a long absence threshold.
- Copy must not blame the user or imply obligation.
- Cooldown prevents repeated return bubbles.

## Affection And Bond

- Repeated local interactions can change phrasing or micro reaction.
- No numeric bond score is shown in UI.
- Phrase pools must avoid guilt, dependency, commands, or emotional coercion.

## Mood Animation

- Happy, sleepy, bored, and calm moods map to distinct animation candidates.
- Missing resource animations fall back to an available safe animation.
- Reduced motion keeps any find beacon static.

## Verification Evidence

- Rust behavior and policy tests cover sleep/wake boundaries, daily greeting, absence return, affection copy, and mood animation fallback.
- Desktop-visible changes still require a Tauri shell launch check before an SDD is accepted.
