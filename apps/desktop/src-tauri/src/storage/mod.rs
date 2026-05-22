pub mod config;
pub mod db;
pub mod migrations;
pub mod repositories;

use std::{fs, io, path::PathBuf};

use crate::{
    protocol::{IanEvent, IanState},
    storage::repositories::memory_repo::MemoryCandidate,
};

pub struct StorageService {
    app_dir: Option<PathBuf>,
}

impl StorageService {
    pub fn initialize() -> io::Result<Self> {
        let app_dir = config::ian_app_dir()?;
        fs::create_dir_all(&app_dir)?;
        config::ensure_default_config(&app_dir)?;
        let database = db::Database::open(&app_dir)?;
        migrations::MigrationRunner::default().run(&database)?;

        Ok(Self {
            app_dir: Some(app_dir),
        })
    }

    pub fn in_memory() -> Self {
        Self { app_dir: None }
    }

    pub fn load_state(&self) -> io::Result<IanState> {
        match &self.app_dir {
            Some(app_dir) => config::load_state(app_dir),
            None => Ok(IanState::default()),
        }
    }

    pub fn persist_state(&self, state: &IanState) -> io::Result<()> {
        match &self.app_dir {
            Some(app_dir) => config::persist_state(app_dir, state),
            None => Ok(()),
        }
    }

    pub fn record_interaction_event(&self, event: &IanEvent) -> io::Result<()> {
        let Some(app_dir) = &self.app_dir else {
            return Ok(());
        };

        let database = db::Database::open(app_dir)?;
        let repository = repositories::event_repo::EventRepository::new(database.connection());
        repository
            .record(event.event_type(), event)
            .map_err(|error| io::Error::new(io::ErrorKind::Other, error))
    }

    pub fn record_life_event(
        &self,
        event_type: &str,
        payload_json: &str,
        created_at_ms: i64,
    ) -> io::Result<()> {
        let Some(app_dir) = &self.app_dir else {
            return Ok(());
        };

        let database = db::Database::open(app_dir)?;
        let repository =
            repositories::life_event_repo::LifeEventRepository::new(database.connection());
        repository
            .record(event_type, payload_json, created_at_ms)
            .map_err(|error| io::Error::new(io::ErrorKind::Other, error))
    }

    pub fn memory_candidates(&self) -> io::Result<Vec<MemoryCandidate>> {
        let Some(app_dir) = &self.app_dir else {
            return Ok(Vec::new());
        };

        let database = db::Database::open(app_dir)?;
        let repository = repositories::memory_repo::MemoryRepository::new(database.connection());
        repository
            .candidates()
            .map_err(|error| io::Error::new(io::ErrorKind::Other, error))
    }

    pub fn confirm_memory_candidate(&self, id: i64, confirmed_at_ms: i64) -> io::Result<()> {
        let Some(app_dir) = &self.app_dir else {
            return Ok(());
        };

        let database = db::Database::open(app_dir)?;
        let repository = repositories::memory_repo::MemoryRepository::new(database.connection());
        repository
            .confirm(id, confirmed_at_ms)
            .map_err(|error| io::Error::new(io::ErrorKind::Other, error))
    }

    pub fn delete_memory_candidate(&self, id: i64) -> io::Result<()> {
        let Some(app_dir) = &self.app_dir else {
            return Ok(());
        };

        let database = db::Database::open(app_dir)?;
        let repository = repositories::memory_repo::MemoryRepository::new(database.connection());
        repository
            .delete(id)
            .map_err(|error| io::Error::new(io::ErrorKind::Other, error))
    }

    pub fn clear_memory_candidates(&self) -> io::Result<()> {
        let Some(app_dir) = &self.app_dir else {
            return Ok(());
        };

        let database = db::Database::open(app_dir)?;
        let repository = repositories::memory_repo::MemoryRepository::new(database.connection());
        repository
            .clear_candidates()
            .map_err(|error| io::Error::new(io::ErrorKind::Other, error))
    }

    pub fn memory_export_summary(&self) -> io::Result<Vec<MemoryCandidate>> {
        let Some(app_dir) = &self.app_dir else {
            return Ok(Vec::new());
        };

        let database = db::Database::open(app_dir)?;
        let repository = repositories::memory_repo::MemoryRepository::new(database.connection());
        repository
            .all()
            .map_err(|error| io::Error::new(io::ErrorKind::Other, error))
    }

    pub fn clear_interaction_journal(&self) -> io::Result<()> {
        let Some(app_dir) = &self.app_dir else {
            return Ok(());
        };

        let database = db::Database::open(app_dir)?;
        let repository = repositories::event_repo::EventRepository::new(database.connection());
        repository
            .clear_all()
            .map_err(|error| io::Error::new(io::ErrorKind::Other, error))
    }

    pub fn confirmed_memory_tags(&self) -> io::Result<Vec<String>> {
        let Some(app_dir) = &self.app_dir else {
            return Ok(Vec::new());
        };

        let now_ms = chrono::Utc::now().timestamp_millis();
        let database = db::Database::open(app_dir)?;
        let repository = repositories::memory_repo::MemoryRepository::new(database.connection());
        let records = repository
            .confirmed(0, now_ms)
            .map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;

        Ok(records
            .into_iter()
            .flat_map(|record| {
                record
                    .tags
                    .split(',')
                    .map(str::to_string)
                    .collect::<Vec<String>>()
            })
            .take(4)
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::{
        db::Database, migrations::MigrationRunner, repositories::event_repo::EventRepository,
    };
    use crate::protocol::IanEvent;

    #[test]
    fn migrations_are_idempotent_and_event_repository_records_interactions() {
        let dir = tempdir().expect("temp dir");
        let database = Database::open(dir.path()).expect("open db");
        let runner = MigrationRunner::default();

        runner.run(&database).expect("first migration");
        runner.run(&database).expect("second migration");

        let repository = EventRepository::new(database.connection());
        repository
            .record("mouse.click", &IanEvent::MouseClick { x: 1.0, y: 2.0 })
            .expect("record event");

        assert_eq!(repository.count_by_type("mouse.click").expect("count"), 1);
    }
}
