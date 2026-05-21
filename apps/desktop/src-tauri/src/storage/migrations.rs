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
