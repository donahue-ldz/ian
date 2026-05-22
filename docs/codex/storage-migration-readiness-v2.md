# Storage Migration Readiness v2

Current decision: storage migration readiness is sufficient for v0.1 skeleton work, not a signal to enable future products.

Readiness map:

- `schema_migrations` records migration state.
- config.toml stores user-editable local settings.
- SQLite repositories keep local-first records.
- memory, social, reminder, visit, mood, and bond repositories are bounded by low-sensitive schemas.
- future schema expansion must be additive or migrated explicitly.

Risk:

- storage readiness must not be interpreted as product readiness for long-term memory, social, plugins, or telemetry.

