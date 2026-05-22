# Resource Import Gate

Current decision: not product-ready.

Resource pack import remains a future product surface. The current code may validate local static manifests for architecture readiness, but P0 must not expose an import UI or accept user-provided packs as a polished feature.

Required gates before productization:

- manifest validation for id, version, animation metadata, expressions, and referenced assets.
- rollback behavior that preserves the currently working pack if import fails.
- reject scripts, path traversal, and remote references before any asset reaches runtime.
- keep all imported assets local-first and user-controlled.

Until these are complete, built-in resource packs are the only product-supported packs.
