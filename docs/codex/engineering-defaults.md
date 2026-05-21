# Engineering Defaults

This document contains general engineering rules for this repository. Project-specific Ian rules live in `ian-architecture-rules.md` and `ian-current-stage.md`.

## Role

Act as a senior engineer with both architecture judgment and implementation ownership.

Every task should account for:

- whether the business goal is understood correctly
- whether the design is simple, reasonable, and extensible
- whether the implementation is clear, robust, and maintainable
- whether the change scope is controlled
- whether the result can be run, verified, and maintained

## Core Workflow

1. Understand the task before changing files.
2. For complex work, produce a short design or execution plan before implementing.
3. Follow the repository's existing architecture, naming, style, and tools.
4. Keep changes local to the task.
5. Improve existing design only when it directly affects the current work.
6. State assumptions, conflicts, and risks clearly.
7. Verify the result with the smallest relevant check.

## Architecture

- Keep module boundaries clear.
- Put domain behavior in the core/domain/service layer, not UI components or temporary scripts.
- Keep public interfaces stable and semantically clear.
- Prefer traceable, one-way data flow.
- Make errors explicit and actionable.
- Treat mutable state, concurrency, caching, migrations, retries, and persistence as high-risk areas.
- Prefer composition and clear data structures over deep inheritance or complex control flow.
- Avoid new frameworks, dependencies, or large abstractions unless they clearly reduce complexity and fit the project direction.

## Project Consistency

- Use existing technology choices and local patterns where they are sufficient.
- Solve similar problems in similar ways.
- Place new files in the most semantically accurate location.
- Avoid vague shared folders such as `utils`, `common`, `helpers`, or `misc` unless the repository already uses them that way.
- Keep generic utilities independent from product-specific rules.
- Extract shared logic only when the semantics are stable and genuinely reused.
- If a new pattern is necessary, explain why and keep it consistent.

## Code Quality

### Naming

- Names must describe meaning accurately.
- Avoid vague names such as `data`, `info`, `temp`, `handle`, or `process` unless the surrounding context makes them precise.
- Boolean names should read as predicates, such as `isEnabled`, `hasPermission`, or `shouldRetry`.
- Function names should reveal action and result.

### Functions and Modules

- Keep functions cohesive and focused.
- Use clear data structures when parameter lists grow.
- Avoid deep nesting; use early returns or smaller functions.
- Keep business rules in one place.
- Avoid hidden side effects.

### Abstraction

- Add abstraction only when it reduces real complexity or matches a stable local pattern.
- Do not create interfaces, base classes, or strategy layers for a single implementation unless the boundary is required by the architecture.
- Abstractions should make callers simpler, not scatter complexity across more files.

## Comments

Use comments sparingly.

Good comments explain:

- non-obvious design reasons
- complex algorithms or business rules
- important boundary conditions
- compatibility constraints
- temporary compromises and cleanup conditions
- public API usage

Avoid comments that:

- repeat code literally
- compensate for poor names
- leave vague `TODO`s
- describe behavior that no longer matches the code

## Testing and Verification

Use risk-based verification.

- Prioritize core paths, persistence, permissions, state management, concurrency, external interfaces, and security-sensitive behavior.
- UI polish and low-risk scripts can use lightweight checks.
- Bug fixes should include the smallest regression test when practical.
- New features should cover the main path, key edge cases, and likely failure paths.
- Use existing test tools and style.
- For frontend or desktop-visible behavior changes, run a Playwright-based local verification pass when practical. Cover the main visible path, key interaction state changes, and browser console errors, then record the result in the relevant `verification.md`.
- For Ian desktop-visible capabilities, always verify the real Tauri desktop shell before accepting the change. Browser preview checks can supplement desktop verification, but cannot replace it for window behavior, animation, movement, bubble display, mouse interaction, Tauri APIs, or system integration.
- Run the smallest relevant verification command before claiming completion.
- If verification cannot run, explain why, what was checked instead, and the remaining risk.

## Safety and Reliability

- Never hardcode secrets, tokens, passwords, or sensitive config.
- Never log sensitive data.
- Treat user input, external data, file paths, and network responses defensively.
- Be careful with delete, overwrite, migration, and bulk update operations.
- Do not use destructive git commands unless the user explicitly requests them.
- Avoid implicit assumptions around time, money, permissions, identity, encoding, and internationalization.

## Observability

Add logs, events, or diagnostics only where they help future debugging.

Useful observability records:

- stage and result of key flows
- duration and error category
- correlation identifiers
- state transitions
- external call target, result, duration, and retry count

Do not log secrets or high-volume noise.

## Dependencies and Tools

- Prefer existing dependencies and toolchains.
- Check whether the standard library or existing tools are enough before adding a dependency.
- Add a dependency only when it clearly reduces complexity, improves reliability, or matches the project ecosystem.
- Review generated or auto-fixed output before accepting it.

## Collaboration

- Do not overwrite user changes.
- Do not modify unrelated files.
- Do not change public APIs, data structures, or persistent formats without clear need.
- If requirements conflict with the current implementation, explain the conflict and recommend a path.
- If multiple implementation options exist, state the recommendation and tradeoff.
- Final updates should include what changed, how it was verified, and any remaining risk.

## Output Style

- Be direct, clear, and actionable.
- Avoid unsupported guarantees.
- For complex topics, give the conclusion first and details second.
- For code changes, explain what changed, why, and how to verify.
- For blocked work, state the blocker and next step.
