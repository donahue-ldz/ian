# Resource Pack Semantic Coverage

Current decision: semantic coverage is auditable and local-first.

Resource packs may directly implement semantic coverage for `idle`, `run`, `happy`, `sleep`, `wake`, `find`, `wave`, and `tantrum`. When a built-in pack lacks a direct semantic animation, Ian must use a local fallback animation instead of failing at runtime.

Rules:

- semantic coverage is checked from local manifest data.
- missing semantics use explicit fallback rows.
- resource packs do not execute scripts, fetch remote references, or own behavior logic.
- future imported resource packs remain default-off until the product gate is ready.

