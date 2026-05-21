# Spec-Driven Development

This directory stores SDD packets generated and maintained by the agent.

The user is not expected to hand-write these documents. The agent creates them before implementation and updates them as decisions change.

## Directory Layout

```txt
docs/sdd/
  README.md
  templates/
    spec.md
    plan.md
    decisions.md
    verification.md
  specs/
    0001-p0-local-creature-proof/
      spec.md
      plan.md
      decisions.md
      verification.md
```

## Packet Rules

Each implementation effort gets one packet under `docs/sdd/specs/`.

Use four-digit incremental ids and short slugs:

```txt
0001-p0-local-creature-proof
0002-resource-pack-renderer
0003-dialogue-provider-boundary
```

Each packet should be self-contained enough that a new agent can understand:

- what is being built
- why it matters
- what is explicitly out of scope
- how it maps to Ian's current stage
- how implementation should proceed
- how completion will be verified

## Stage Discipline

P0 packets must not accidentally pull in v0.2 or v0.3 features.

If a future feature is useful for architecture, represent it as a skeleton, trait, interface, schema placeholder, or explicit non-goal. Do not expose it as a user-visible product feature unless the current-stage document allows it.

