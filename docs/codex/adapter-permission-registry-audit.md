# Adapter Permission Registry Audit

Current decision: adapter permissions are explicit and unknown sources are denied.

Registry boundaries:

- low-sensitive local sources can be allowed by policy.
- Git, build/test, keyboard rhythm, and active app presence are default-off.
- clipboard, screen OCR, code body, private chat content, and terminal full output are denied.
- adapters may collect signals only after permission and sanitizer checks.
- adapters must not directly control animations or persistent state.

