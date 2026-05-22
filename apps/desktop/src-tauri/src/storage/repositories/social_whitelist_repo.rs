use chrono::Utc;
use rusqlite::{params, Connection};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SocialWhitelistEntry {
    pub source_id: String,
    pub display_name: String,
    pub allowed: bool,
    pub updated_at_ms: i64,
}

pub struct SocialWhitelistRepository<'a> {
    connection: &'a Connection,
}

impl<'a> SocialWhitelistRepository<'a> {
    pub fn new(connection: &'a Connection) -> Self {
        Self { connection }
    }

    pub fn allow(
        &self,
        source_id: &str,
        display_name: &str,
        updated_at_ms: i64,
    ) -> rusqlite::Result<()> {
        self.connection.execute(
            "INSERT INTO social_whitelist (source_id, display_name, allowed, updated_at_ms, updated_at) VALUES (?1, ?2, 1, ?3, ?4)
             ON CONFLICT(source_id) DO UPDATE SET display_name = excluded.display_name, allowed = 1, updated_at_ms = excluded.updated_at_ms, updated_at = excluded.updated_at",
            params![source_id, display_name, updated_at_ms, Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn revoke(&self, source_id: &str, updated_at_ms: i64) -> rusqlite::Result<()> {
        self.connection.execute(
            "UPDATE social_whitelist SET allowed = 0, updated_at_ms = ?2, updated_at = ?3 WHERE source_id = ?1",
            params![source_id, updated_at_ms, Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn is_allowed(&self, source_id: &str) -> rusqlite::Result<bool> {
        let mut statement = self
            .connection
            .prepare("SELECT allowed FROM social_whitelist WHERE source_id = ?1")?;
        let result = statement.query_row(params![source_id], |row| row.get::<_, i64>(0));
        match result {
            Ok(value) => Ok(value == 1),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(false),
            Err(error) => Err(error),
        }
    }

    pub fn allowed_entries(&self) -> rusqlite::Result<Vec<SocialWhitelistEntry>> {
        let mut statement = self.connection.prepare(
            "SELECT source_id, display_name, allowed, updated_at_ms FROM social_whitelist WHERE allowed = 1 ORDER BY updated_at_ms ASC",
        )?;
        let rows = statement.query_map([], |row| {
            Ok(SocialWhitelistEntry {
                source_id: row.get(0)?,
                display_name: row.get(1)?,
                allowed: row.get::<_, i64>(2)? == 1,
                updated_at_ms: row.get(3)?,
            })
        })?;

        rows.collect()
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use crate::storage::{db::Database, migrations::MigrationRunner};

    use super::SocialWhitelistRepository;

    #[test]
    fn social_whitelist_defaults_empty_and_revoke_stops_future_access() {
        let dir = tempdir().expect("temp dir");
        let database = Database::open(dir.path()).expect("open db");
        MigrationRunner::default().run(&database).expect("migrate");
        let repository = SocialWhitelistRepository::new(database.connection());

        assert!(!repository.is_allowed("remote-1").expect("default denied"));
        assert!(repository.allowed_entries().expect("empty").is_empty());

        repository
            .allow("remote-1", "Friend", 1_000)
            .expect("allow");
        assert!(repository.is_allowed("remote-1").expect("allowed"));

        repository.revoke("remote-1", 2_000).expect("revoke");
        assert!(!repository.is_allowed("remote-1").expect("revoked"));
        assert!(repository.allowed_entries().expect("none").is_empty());
    }
}
