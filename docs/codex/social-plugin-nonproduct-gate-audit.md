# Social Plugin Nonproduct Gate Audit

Current decision: social and plugin systems are not product-ready.

Social gate:

- no default network request.
- Feishu and Pet Visit remain skeleton-only.
- high-sensitive payloads are rejected or sanitized before IanEvent.

Plugin gate:

- no plugin execution entry.
- no script execution.
- unknown permissions are rejected.
- plugin loading remains default-off.

