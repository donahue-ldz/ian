pub mod config;
pub mod db;
pub mod migrations;
pub mod repositories;

use std::{fs, io, path::PathBuf};

use crate::protocol::IanState;

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
}
