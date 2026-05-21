# Playful Expressiveness Acceptance Checklist

## Scope

This checklist validates the 0061-0069 playful-expression slice as a single user experience. It checks that Ian can be lively, surprising, and affectionate while remaining bounded, local, quietable, and user-controlled.

## Commands

```bash
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npm run desktop:test
npm run desktop:typecheck
npm run desktop:build
```

## Zoomies

Scenario: set playful energy to high, let Rust Core receive an eligible idle tick, and inspect the returned action sequence.

Pass condition: the sequence includes `playful.diagnostic`, `behavior.zoomies`, `effect.play`, `animation.play` with `zoomies`, 4-8 bounded `movement.move_to` actions, and an idle/anchor return within the configured duration.

Fail condition: zoomies starts without a diagnostic reason, emits fewer than 4 or more than 8 waypoints, runs outside movement boundaries, or does not settle back to idle/anchor.

## Cute Reactions

Scenario: click Ian repeatedly from a normal idle state and observe short speech, animation, and light effects.

Pass condition: local affectionate phrases vary across repeated interactions, visible reactions include at least `heart_pop`, `sparkle_pop`, and `blush_puff`, and repeated clicks are rate-limited instead of flooding the bubble.

Fail condition: the same phrase loops mechanically, visible feedback spams continuously, a Mood/Bond number appears, or Ian uses assistant-like wording.

## Controlled Randomness

Scenario: run deterministic Rust tests and compare two seeded playful candidates.

Pass condition: fixed seeds reproduce the same candidate sequence, runtime action output still passes cooldown and movement-boundary policy, and React contains no behavior-random decision source.

Fail condition: tests depend on flaky randomness, React uses random choice to decide Ian behavior, or random candidates bypass safety gates.

## Safety Gates

Scenario: repeat the idle tick with `playful_energy = off`, quiet mode, drag/input active, and active cooldown.

Pass condition: off, quiet, drag/input active state, snooze, and cooldown block spontaneous high-energy behavior while ordinary click feedback still works.

Fail condition: spontaneous zoomies starts while disabled or quiet, starts during user input/dragging, stacks during cooldown, or blocks normal click affection.

## Visual Effects

Scenario: inspect resource packs, CSS, and browser DOM during playful interactions.

Pass condition: zoomies is visually distinct from run, local `effect.play` renders a nonblocking `.ian-visual-effect`, reduced motion hides or lowers the effect, and missing resources fall back without blanking.

Fail condition: zoomies looks identical to run, visual effects cover the bubble/settings entry, reduced motion still shows high-intensity effects, or missing animation data crashes rendering.

## Evidence

Scenario: collect automated test output, browser smoke JSON, and at least one browser screenshot.

Pass condition: verification records command results, browser smoke fields for settings/click/zoomies/safety, screenshot path, and a short human conclusion about "可爱但不烦".

Fail condition: verification only says tests passed without command output, omits Browser/Tauri smoke, or leaves subjective conclusions without observable evidence.
