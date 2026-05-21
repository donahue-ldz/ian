use chrono::Utc;
use rusqlite::{params, Connection};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifeEventRecord {
    pub event_type: String,
    pub payload_json: String,
    pub created_at_ms: i64,
}

pub struct LifeEventRepository<'a> {
    connection: &'a Connection,
}

impl<'a> LifeEventRepository<'a> {
    pub fn new(connection: &'a Connection) -> Self {
        Self { connection }
    }

    pub fn record(
        &self,
        event_type: &str,
        payload_json: &str,
        created_at_ms: i64,
    ) -> rusqlite::Result<()> {
        self.connection.execute(
            "INSERT INTO life_events (event_type, payload_json, created_at_ms, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![event_type, payload_json, created_at_ms, Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn list_between(
        &self,
        start_ms: i64,
        end_ms: i64,
    ) -> rusqlite::Result<Vec<LifeEventRecord>> {
        let mut statement = self.connection.prepare(
            "SELECT event_type, payload_json, created_at_ms FROM life_events WHERE created_at_ms >= ?1 AND created_at_ms <= ?2 ORDER BY created_at_ms ASC",
        )?;
        let rows = statement.query_map(params![start_ms, end_ms], |row| {
            Ok(LifeEventRecord {
                event_type: row.get(0)?,
                payload_json: row.get(1)?,
                created_at_ms: row.get(2)?,
            })
        })?;

        rows.collect()
    }

    pub fn interaction_summary(&self, now_ms: i64) -> rusqlite::Result<InteractionSummary> {
        let mut statement = self.connection.prepare(
            "SELECT event_type, created_at_ms FROM life_events WHERE event_type IN ('interaction.click', 'interaction.double_click', 'dialogue.reply')",
        )?;
        let rows = statement.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })?;

        let mut summary = InteractionSummary::default();
        for row in rows {
            let (_, created_at_ms) = row?;
            summary.interaction_count += 1;
            summary.last_interaction_at_ms = Some(
                summary
                    .last_interaction_at_ms
                    .map_or(created_at_ms, |current| current.max(created_at_ms)),
            );
        }
        summary.recent_activity_level = summary.activity_level(now_ms);
        Ok(summary)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InteractionSummary {
    pub interaction_count: u32,
    pub last_interaction_at_ms: Option<i64>,
    pub recent_activity_level: String,
}

impl InteractionSummary {
    fn activity_level(&self, now_ms: i64) -> String {
        let Some(last_at_ms) = self.last_interaction_at_ms else {
            return "none".to_string();
        };

        if now_ms.saturating_sub(last_at_ms) <= 30 * 60 * 1000 {
            "recent".to_string()
        } else {
            "quiet".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use tempfile::tempdir;

    use crate::storage::{db::Database, migrations::MigrationRunner};

    use super::LifeEventRepository;

    #[test]
    fn life_events_append_and_read_in_time_order() {
        let dir = tempdir().expect("temp dir");
        let database = Database::open(dir.path()).expect("open db");
        MigrationRunner::default().run(&database).expect("migrate");
        let repository = LifeEventRepository::new(database.connection());

        repository
            .record("life.started", r#"{"source":"runtime"}"#, 1_000)
            .expect("record start");
        repository
            .record("movement.move", r#"{"action":"movement.move_to"}"#, 2_000)
            .expect("record movement");

        let events = repository.list_between(0, 3_000).expect("list life events");

        assert_eq!(events.len(), 2);
        assert_eq!(events[0].event_type, "life.started");
        assert_eq!(events[1].event_type, "movement.move");
    }

    #[test]
    fn life_event_payload_is_low_sensitive_allowlist() {
        let dir = tempdir().expect("temp dir");
        let database = Database::open(dir.path()).expect("open db");
        MigrationRunner::default().run(&database).expect("migrate");
        let repository = LifeEventRepository::new(database.connection());

        repository
            .record(
                "interaction.click",
                r#"{"event":"mouse.click","action_count":3}"#,
                1_000,
            )
            .expect("record click");

        let events = repository.list_between(0, 2_000).expect("list events");
        let payload: Value = serde_json::from_str(&events[0].payload_json).expect("parse payload");

        assert!(payload.get("event").is_some());
        assert!(payload.get("text").is_none());
        assert!(payload.get("window_title").is_none());
        assert!(payload.get("code").is_none());
    }

    #[test]
    fn interaction_summary_uses_counts_and_times_without_text() {
        let dir = tempdir().expect("temp dir");
        let database = Database::open(dir.path()).expect("open db");
        MigrationRunner::default().run(&database).expect("migrate");
        let repository = LifeEventRepository::new(database.connection());

        repository
            .record("interaction.click", r#"{"event":"mouse.click"}"#, 1_000)
            .expect("record click");
        repository
            .record("dialogue.reply", r#"{"reply_kind":"demo"}"#, 2_000)
            .expect("record dialogue");

        let summary = repository
            .interaction_summary(20_000)
            .expect("interaction summary");

        assert_eq!(summary.interaction_count, 2);
        assert_eq!(summary.last_interaction_at_ms, Some(2_000));
        assert_eq!(summary.recent_activity_level, "recent");
    }
}
