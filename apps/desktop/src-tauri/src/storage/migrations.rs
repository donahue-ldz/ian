use std::io;

use super::db::Database;

#[derive(Default)]
pub struct MigrationRunner;

impl MigrationRunner {
    pub fn run(&self, database: &Database) -> io::Result<()> {
        database
            .connection()
            .execute_batch(
                "
                CREATE TABLE IF NOT EXISTS schema_migrations (
                  version INTEGER PRIMARY KEY,
                  applied_at TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS pet_identities (
                  id TEXT PRIMARY KEY,
                  display_name TEXT NOT NULL,
                  created_at TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS resource_packs (
                  id TEXT PRIMARY KEY,
                  version TEXT NOT NULL,
                  path TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS interaction_events (
                  id INTEGER PRIMARY KEY AUTOINCREMENT,
                  event_type TEXT NOT NULL,
                  payload_json TEXT NOT NULL,
                  created_at TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS life_events (
                  id INTEGER PRIMARY KEY AUTOINCREMENT,
                  event_type TEXT NOT NULL,
                  payload_json TEXT NOT NULL,
                  created_at_ms INTEGER NOT NULL,
                  created_at TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS reminder_records (
                  id INTEGER PRIMARY KEY AUTOINCREMENT,
                  kind TEXT NOT NULL,
                  outcome TEXT NOT NULL,
                  payload_json TEXT NOT NULL,
                  occurred_at_ms INTEGER NOT NULL,
                  created_at TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS memory_candidates (
                  id INTEGER PRIMARY KEY AUTOINCREMENT,
                  tags TEXT NOT NULL,
                  status TEXT NOT NULL,
                  created_at_ms INTEGER NOT NULL,
                  confirmed_at_ms INTEGER,
                  created_at TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS social_whitelist (
                  source_id TEXT PRIMARY KEY,
                  display_name TEXT NOT NULL,
                  allowed INTEGER NOT NULL,
                  updated_at_ms INTEGER NOT NULL,
                  updated_at TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS visit_records (
                  id INTEGER PRIMARY KEY AUTOINCREMENT,
                  source_id TEXT NOT NULL,
                  action TEXT NOT NULL,
                  outcome TEXT NOT NULL,
                  occurred_at_ms INTEGER NOT NULL,
                  payload_json TEXT NOT NULL,
                  created_at TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS adapter_audit_logs (
                  id INTEGER PRIMARY KEY AUTOINCREMENT,
                  adapter_id TEXT NOT NULL,
                  action TEXT NOT NULL,
                  payload_json TEXT NOT NULL,
                  created_at_ms INTEGER NOT NULL,
                  created_at TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS settings_kv (
                  key TEXT PRIMARY KEY,
                  value TEXT NOT NULL,
                  updated_at TEXT NOT NULL
                );
                ",
            )
            .map_err(|error| io::Error::new(io::ErrorKind::Other, error))
    }
}
