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
}
