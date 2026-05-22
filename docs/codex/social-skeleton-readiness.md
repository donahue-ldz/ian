# Social Skeleton Readiness

Current decision: not product-ready.

Social presence, Feishu relay, group broadcasts, and Pet Visit remain skeleton-only architecture. They must not create default network requests or user-visible social behavior in P0.

Readiness conditions:

- all social skeletons remain disabled by default.
- no default network request may occur from the skeleton.
- inbound text must be allowlisted, short, and low-sensitive before display.
- high-sensitive payloads are rejected or sanitized before becoming IanEvent input.
- Pet Visit playback uses only local allowlisted actions and resources.
