use chrono::Utc;
use rusqlite::{params, Connection};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryCandidate {
    pub id: i64,
    pub tags: String,
    pub status: String,
    pub created_at_ms: i64,
    pub confirmed_at_ms: Option<i64>,
}

pub struct MemoryRepository<'a> {
    connection: &'a Connection,
}

impl<'a> MemoryRepository<'a> {
    pub fn new(connection: &'a Connection) -> Self {
        Self { connection }
    }

    pub fn create_candidate(&self, tags: &str, created_at_ms: i64) -> rusqlite::Result<i64> {
        validate_low_sensitive_tags(tags)?;
        self.connection.execute(
            "INSERT INTO memory_candidates (tags, status, created_at_ms, confirmed_at_ms, created_at) VALUES (?1, 'candidate', ?2, NULL, ?3)",
            params![tags, created_at_ms, Utc::now().to_rfc3339()],
        )?;
        Ok(self.connection.last_insert_rowid())
    }

    pub fn confirm(&self, id: i64, confirmed_at_ms: i64) -> rusqlite::Result<()> {
        self.connection.execute(
            "UPDATE memory_candidates SET status = 'confirmed', confirmed_at_ms = ?2 WHERE id = ?1 AND status = 'candidate'",
            params![id, confirmed_at_ms],
        )?;
        Ok(())
    }

    pub fn delete(&self, id: i64) -> rusqlite::Result<()> {
        self.connection
            .execute("DELETE FROM memory_candidates WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn purge_expired_candidates(&self, older_than_ms: i64) -> rusqlite::Result<usize> {
        self.connection.execute(
            "DELETE FROM memory_candidates WHERE status = 'candidate' AND created_at_ms < ?1",
            params![older_than_ms],
        )
    }

    pub fn clear_all(&self) -> rusqlite::Result<()> {
        self.connection
            .execute("DELETE FROM memory_candidates", [])?;
        Ok(())
    }

    pub fn confirmed(&self, start_ms: i64, end_ms: i64) -> rusqlite::Result<Vec<MemoryCandidate>> {
        let mut statement = self.connection.prepare(
            "SELECT id, tags, status, created_at_ms, confirmed_at_ms FROM memory_candidates WHERE status = 'confirmed' AND created_at_ms >= ?1 AND created_at_ms <= ?2 ORDER BY created_at_ms ASC",
        )?;
        let rows = statement.query_map(params![start_ms, end_ms], |row| {
            Ok(MemoryCandidate {
                id: row.get(0)?,
                tags: row.get(1)?,
                status: row.get(2)?,
                created_at_ms: row.get(3)?,
                confirmed_at_ms: row.get(4)?,
            })
        })?;

        rows.collect()
    }
}

fn validate_low_sensitive_tags(tags: &str) -> rusqlite::Result<()> {
    let is_low_sensitive = !tags.is_empty()
        && tags.len() <= 160
        && !tags.contains("text:")
        && !tags.contains('/')
        && !tags.contains('\\')
        && !tags.contains('<')
        && !tags.contains('>');

    if is_low_sensitive {
        Ok(())
    } else {
        Err(rusqlite::Error::InvalidParameterName(
            "memory candidate must contain only low-sensitive tags".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use crate::storage::{db::Database, migrations::MigrationRunner};

    use super::MemoryRepository;

    #[test]
    fn memory_candidates_require_confirmation_and_keep_only_low_sensitive_tags() {
        let dir = tempdir().expect("temp dir");
        let database = Database::open(dir.path()).expect("open db");
        MigrationRunner::default().run(&database).expect("migrate");
        let repository = MemoryRepository::new(database.connection());

        let id = repository
            .create_candidate("kind:favorite_place,tag:window", 1_000)
            .expect("candidate");
        assert!(repository
            .create_candidate("text:I live at 1 Infinite Loop", 1_100)
            .is_err());

        assert!(repository
            .confirmed(0, 2_000)
            .expect("confirmed")
            .is_empty());
        repository.confirm(id, 1_200).expect("confirm");

        let confirmed = repository.confirmed(0, 2_000).expect("confirmed");
        assert_eq!(confirmed.len(), 1);
        assert_eq!(confirmed[0].tags, "kind:favorite_place,tag:window");
        assert!(!confirmed[0].tags.contains("text:"));
    }

    #[test]
    fn memory_candidates_can_be_deleted_expired_and_cleared() {
        let dir = tempdir().expect("temp dir");
        let database = Database::open(dir.path()).expect("open db");
        MigrationRunner::default().run(&database).expect("migrate");
        let repository = MemoryRepository::new(database.connection());

        let old_id = repository
            .create_candidate("kind:moment,tag:morning", 1_000)
            .expect("old");
        let fresh_id = repository
            .create_candidate("kind:moment,tag:evening", 10_000)
            .expect("fresh");

        repository.delete(old_id).expect("delete old");
        repository.confirm(fresh_id, 11_000).expect("confirm fresh");
        repository
            .purge_expired_candidates(5_000)
            .expect("purge expired");

        let confirmed = repository.confirmed(0, 20_000).expect("confirmed");
        assert_eq!(confirmed.len(), 1);
        assert_eq!(confirmed[0].id, fresh_id);

        repository.clear_all().expect("clear all");
        assert!(repository.confirmed(0, 20_000).expect("empty").is_empty());
    }
}
