# Plugin Skeleton Readiness

Current decision: not product-ready.

The plugin system is architecture skeleton only. There is no plugin execution entry in P0, no default plugin loading, and no script execution path.

Readiness conditions:

- plugin manifests stay default-off.
- unknown permissions are rejected.
- authorized events are explicit and revocable.
- no plugin execution entry exists until a security model, review flow, and user-facing consent surface are approved.
- plugin scripts are not executed by the desktop runtime.
