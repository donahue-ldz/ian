use chrono::Utc;
use rusqlite::{params, Connection};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReminderRecord {
    pub kind: String,
    pub outcome: String,
    pub payload_json: String,
    pub occurred_at_ms: i64,
}

pub struct ReminderRepository<'a> {
    connection: &'a Connection,
}

impl<'a> ReminderRepository<'a> {
    pub fn new(connection: &'a Connection) -> Self {
        Self { connection }
    }

    pub fn record(&self, kind: &str, outcome: &str, occurred_at_ms: i64) -> rusqlite::Result<()> {
        let payload_json = serde_json::json!({
            "kind": kind,
            "outcome": outcome,
        })
        .to_string();
        self.connection.execute(
            "INSERT INTO reminder_records (kind, outcome, payload_json, occurred_at_ms, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![kind, outcome, payload_json, occurred_at_ms, Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn recent(&self, start_ms: i64, end_ms: i64) -> rusqlite::Result<Vec<ReminderRecord>> {
        let mut statement = self.connection.prepare(
            "SELECT kind, outcome, payload_json, occurred_at_ms FROM reminder_records WHERE occurred_at_ms >= ?1 AND occurred_at_ms <= ?2 ORDER BY occurred_at_ms ASC",
        )?;
        let rows = statement.query_map(params![start_ms, end_ms], |row| {
            Ok(ReminderRecord {
                kind: row.get(0)?,
                outcome: row.get(1)?,
                payload_json: row.get(2)?,
                occurred_at_ms: row.get(3)?,
            })
        })?;

        rows.collect()
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use crate::storage::{db::Database, migrations::MigrationRunner};

    use super::ReminderRepository;

    #[test]
    fn reminder_records_append_recent_and_avoid_user_text() {
        let dir = tempdir().expect("temp dir");
        let database = Database::open(dir.path()).expect("open db");
        MigrationRunner::default().run(&database).expect("migrate");
        let repository = ReminderRepository::new(database.connection());

        repository
            .record("hydration", "confirmed", 1_000)
            .expect("record reminder");
        let records = repository.recent(0, 2_000).expect("recent reminders");

        assert_eq!(records.len(), 1);
        assert_eq!(records[0].kind, "hydration");
        assert_eq!(records[0].outcome, "confirmed");
        assert!(!records[0].payload_json.contains("text"));
    }
}
