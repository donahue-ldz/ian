pub mod config;
pub mod db;
pub mod migrations;
pub mod repositories;

use std::{fs, io, path::PathBuf};

use crate::protocol::{IanEvent, IanState};

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
