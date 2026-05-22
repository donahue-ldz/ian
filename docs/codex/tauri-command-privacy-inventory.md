# Tauri Command Privacy Inventory

Current decision: Tauri commands expose local Ian control surfaces and must keep inputs low-sensitive.

Command inventory:

- `handle_ian_event`: accepts structured IanEvent payloads after Security Gate inspection.
- `get_ian_state` and `get_settings`: return local IanState.
- `save_window_position` and `reset_window_position`: persist local position only.
- `save_creature_settings`, `save_behavior_mode`, `save_quiet_hours`, and `save_do_not_disturb`: persist local settings.
- memory commands expose low-sensitive tags and review actions.
- developer commands remain bounded by workspace binding and default-off settings.

Disallowed by this inventory:

- arbitrary file reads.
- clipboard reads.
- screen OCR.
- code body or diff reads.
- terminal full output reads.

