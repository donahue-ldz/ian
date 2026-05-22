use chrono::Utc;
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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

    pub fn candidates(&self) -> rusqlite::Result<Vec<MemoryCandidate>> {
        self.list_by_status("candidate")
    }

    pub fn all(&self) -> rusqlite::Result<Vec<MemoryCandidate>> {
        let mut statement = self.connection.prepare(
            "SELECT id, tags, status, created_at_ms, confirmed_at_ms FROM memory_candidates ORDER BY created_at_ms ASC",
        )?;
        let rows = statement.query_map([], memory_candidate_from_row)?;

        rows.collect()
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

    pub fn clear_candidates(&self) -> rusqlite::Result<()> {
        self.connection.execute(
            "DELETE FROM memory_candidates WHERE status = 'candidate'",
            [],
        )?;
        Ok(())
    }

    pub fn confirmed(&self, start_ms: i64, end_ms: i64) -> rusqlite::Result<Vec<MemoryCandidate>> {
        let mut statement = self.connection.prepare(
            "SELECT id, tags, status, created_at_ms, confirmed_at_ms FROM memory_candidates WHERE status = 'confirmed' AND created_at_ms >= ?1 AND created_at_ms <= ?2 ORDER BY created_at_ms ASC",
        )?;
        let rows = statement.query_map(params![start_ms, end_ms], memory_candidate_from_row)?;

        rows.collect()
    }

    fn list_by_status(&self, status: &str) -> rusqlite::Result<Vec<MemoryCandidate>> {
        let mut statement = self.connection.prepare(
            "SELECT id, tags, status, created_at_ms, confirmed_at_ms FROM memory_candidates WHERE status = ?1 ORDER BY created_at_ms ASC",
        )?;
        let rows = statement.query_map(params![status], memory_candidate_from_row)?;

        rows.collect()
    }
}

fn memory_candidate_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<MemoryCandidate> {
    Ok(MemoryCandidate {
        id: row.get(0)?,
        tags: row.get(1)?,
        status: row.get(2)?,
        created_at_ms: row.get(3)?,
        confirmed_at_ms: row.get(4)?,
    })
}

fn validate_low_sensitive_tags(tags: &str) -> rusqlite::Result<()> {
    let is_low_sensitive = !tags.is_empty()
        && tags.len() <= 160
        && tags.split(',').all(is_allowed_memory_tag)
        && !looks_sensitive(tags);

    if is_low_sensitive {
        Ok(())
    } else {
        Err(rusqlite::Error::InvalidParameterName(
            "memory candidate must contain only low-sensitive tags".to_string(),
        ))
    }
}

fn is_allowed_memory_tag(tag: &str) -> bool {
    let Some((prefix, value)) = tag.split_once(':') else {
        return false;
    };
    if !matches!(prefix, "pref" | "topic" | "routine" | "moment" | "place") {
        return false;
    }

    !value.is_empty()
        && value.len() <= 48
        && value
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_' || ch == '-')
}

fn looks_sensitive(value: &str) -> bool {
    value.contains("text:")
        || value.contains("code:")
        || value.contains("html:")
        || value.contains('/')
        || value.contains('\\')
        || value.contains('<')
        || value.contains('>')
        || value.contains('{')
        || value.contains('}')
        || value.contains("fn ")
        || value.contains("secret")
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
            .create_candidate("place:window,topic:water", 1_000)
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
        assert_eq!(confirmed[0].tags, "place:window,topic:water");
        assert!(!confirmed[0].tags.contains("text:"));
    }

    #[test]
    fn memory_candidates_can_be_deleted_expired_and_cleared() {
        let dir = tempdir().expect("temp dir");
        let database = Database::open(dir.path()).expect("open db");
        MigrationRunner::default().run(&database).expect("migrate");
        let repository = MemoryRepository::new(database.connection());

        let old_id = repository
            .create_candidate("moment:morning", 1_000)
            .expect("old");
        let fresh_id = repository
            .create_candidate("moment:evening", 10_000)
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

    #[test]
    fn memory_candidate_review_lists_candidates_and_keeps_confirmed_records() {
        let dir = tempdir().expect("temp dir");
        let database = Database::open(dir.path()).expect("open db");
        MigrationRunner::default().run(&database).expect("migrate");
        let repository = MemoryRepository::new(database.connection());

        let first = repository
            .create_candidate("pref:quiet,topic:water", 1_000)
            .expect("first");
        let second = repository
            .create_candidate("routine:stretch", 2_000)
            .expect("second");
        repository.confirm(second, 2_500).expect("confirm second");

        let candidates = repository.candidates().expect("candidates");
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].id, first);
        assert_eq!(candidates[0].tags, "pref:quiet,topic:water");
        assert_eq!(candidates[0].status, "candidate");

        repository.clear_candidates().expect("clear candidates");
        assert!(repository
            .candidates()
            .expect("empty candidates")
            .is_empty());
        assert_eq!(repository.confirmed(0, 3_000).expect("confirmed").len(), 1);
    }

    #[test]
    fn low_sensitive_memory_tags_reject_paths_code_html_and_free_text() {
        let dir = tempdir().expect("temp dir");
        let database = Database::open(dir.path()).expect("open db");
        MigrationRunner::default().run(&database).expect("migrate");
        let repository = MemoryRepository::new(database.connection());

        for rejected in [
            "/Users/alice/project",
            "code:fn main() {}",
            "html:<b>secret</b>",
            "text:likes private chat",
            "pref:quiet,unknown:raw",
            "pref:",
        ] {
            assert!(
                repository.create_candidate(rejected, 1_000).is_err(),
                "{rejected} should be rejected"
            );
        }

        repository
            .create_candidate(
                "pref:quiet,topic:water,routine:stretch,moment:morning",
                1_000,
            )
            .expect("allowed low-sensitive tags");
    }
}
