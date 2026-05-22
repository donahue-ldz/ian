use chrono::Utc;
use rusqlite::{params, Connection};
use serde::Serialize;

pub struct EventRepository<'a> {
    connection: &'a Connection,
}

impl<'a> EventRepository<'a> {
    pub fn new(connection: &'a Connection) -> Self {
        Self { connection }
    }

    pub fn record<T: Serialize>(&self, event_type: &str, payload: &T) -> rusqlite::Result<()> {
        let payload_json = serde_json::to_string(payload)
            .unwrap_or_else(|_| "{\"error\":\"payload_serialize_failed\"}".to_string());
        self.connection.execute(
            "INSERT INTO interaction_events (event_type, payload_json, created_at) VALUES (?1, ?2, ?3)",
            params![event_type, payload_json, Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn count_by_type(&self, event_type: &str) -> rusqlite::Result<u64> {
        self.connection.query_row(
            "SELECT COUNT(*) FROM interaction_events WHERE event_type = ?1",
            params![event_type],
            |row| row.get::<_, u64>(0),
        )
    }

    pub fn clear_all(&self) -> rusqlite::Result<()> {
        self.connection
            .execute("DELETE FROM interaction_events", [])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use crate::{
        protocol::IanEvent,
        storage::{db::Database, migrations::MigrationRunner},
    };

    use super::EventRepository;

    #[test]
    fn clear_all_removes_local_interaction_journal_entries_independently() {
        let dir = tempdir().expect("temp dir");
        let database = Database::open(dir.path()).expect("open db");
        MigrationRunner::default().run(&database).expect("migrate");
        let repository = EventRepository::new(database.connection());

        repository
            .record("mouse.click", &IanEvent::MouseClick { x: 1.0, y: 2.0 })
            .expect("record click");
        repository
            .record(
                "mouse.near",
                &IanEvent::MouseNear {
                    x: 1.0,
                    y: 2.0,
                    now_ms: 1_000,
                },
            )
            .expect("record near");

        repository.clear_all().expect("clear journal");

        assert_eq!(repository.count_by_type("mouse.click").expect("clicks"), 0);
        assert_eq!(repository.count_by_type("mouse.near").expect("near"), 0);
    }
}
