use chrono::Utc;
use rusqlite::{params, Connection};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisitRecord {
    pub source_id: String,
    pub action: String,
    pub outcome: String,
    pub occurred_at_ms: i64,
    pub payload_json: String,
}

pub struct VisitRecordRepository<'a> {
    connection: &'a Connection,
}

impl<'a> VisitRecordRepository<'a> {
    pub fn new(connection: &'a Connection) -> Self {
        Self { connection }
    }

    pub fn record(
        &self,
        source_id: &str,
        action: &str,
        outcome: &str,
        occurred_at_ms: i64,
    ) -> rusqlite::Result<()> {
        let payload_json = serde_json::json!({
            "source_id": source_id,
            "action": action,
            "outcome": outcome,
        })
        .to_string();
        self.connection.execute(
            "INSERT INTO visit_records (source_id, action, outcome, occurred_at_ms, payload_json, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![source_id, action, outcome, occurred_at_ms, payload_json, Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn recent(&self, start_ms: i64, end_ms: i64) -> rusqlite::Result<Vec<VisitRecord>> {
        let mut statement = self.connection.prepare(
            "SELECT source_id, action, outcome, occurred_at_ms, payload_json FROM visit_records WHERE occurred_at_ms >= ?1 AND occurred_at_ms <= ?2 ORDER BY occurred_at_ms ASC",
        )?;
        let rows = statement.query_map(params![start_ms, end_ms], |row| {
            Ok(VisitRecord {
                source_id: row.get(0)?,
                action: row.get(1)?,
                outcome: row.get(2)?,
                occurred_at_ms: row.get(3)?,
                payload_json: row.get(4)?,
            })
        })?;

        rows.collect()
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use crate::storage::{
        db::Database,
        migrations::MigrationRunner,
        repositories::{
            social_whitelist_repo::SocialWhitelistRepository,
            visit_record_repo::VisitRecordRepository,
        },
    };

    #[test]
    fn visit_records_append_low_sensitive_summary_without_message_body() {
        let dir = tempdir().expect("temp dir");
        let database = Database::open(dir.path()).expect("open db");
        MigrationRunner::default().run(&database).expect("migrate");
        let whitelist = SocialWhitelistRepository::new(database.connection());
        let visits = VisitRecordRepository::new(database.connection());

        whitelist.allow("remote-1", "Friend", 1_000).expect("allow");
        visits
            .record("remote-1", "wave", "played", 2_000)
            .expect("visit");
        whitelist.revoke("remote-1", 3_000).expect("revoke");

        let records = visits.recent(0, 4_000).expect("records");
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].action, "wave");
        assert!(!records[0].payload_json.contains("text"));
        assert!(!whitelist.is_allowed("remote-1").expect("revoked"));
    }
}
