use std::{fs, io, path::Path};

use directories::BaseDirs;
use serde::{Deserialize, Serialize};

use crate::protocol::{BehaviorMode, IanState, Position};

#[derive(Debug, Serialize, Deserialize)]
struct ConfigFile {
    app: AppConfig,
    behavior: BehaviorConfig,
    position: Position,
}

#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    active_pet: String,
    active_resource_pack: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BehaviorConfig {
    mode: BehaviorMode,
}

pub fn ian_app_dir() -> io::Result<std::path::PathBuf> {
    let dirs = BaseDirs::new().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "could not resolve Ian app directory",
        )
    })?;

    Ok(dirs.home_dir().join(".ian"))
}

pub fn ensure_default_config(app_dir: &Path) -> io::Result<()> {
    let config_path = app_dir.join("config.toml");
    if config_path.exists() {
        return Ok(());
    }

    let config = ConfigFile::from(IanState::default());
    let serialized = toml::to_string_pretty(&config)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    fs::write(config_path, serialized)
}

pub fn load_state(app_dir: &Path) -> io::Result<IanState> {
    let config_path = app_dir.join("config.toml");
    if !config_path.exists() {
        return Ok(IanState::default());
    }

    let content = fs::read_to_string(config_path)?;
    let config: ConfigFile = toml::from_str(&content)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;

    Ok(config.into())
}

pub fn persist_state(app_dir: &Path, state: &IanState) -> io::Result<()> {
    let config_path = app_dir.join("config.toml");
    let serialized = toml::to_string_pretty(&ConfigFile::from(state.clone()))
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    fs::write(config_path, serialized)
}

impl From<IanState> for ConfigFile {
    fn from(state: IanState) -> Self {
        Self {
            app: AppConfig {
                active_pet: state.active_pet_id,
                active_resource_pack: state.active_resource_pack,
            },
            behavior: BehaviorConfig {
                mode: state.behavior_mode,
            },
            position: state.position,
        }
    }
}

impl From<ConfigFile> for IanState {
    fn from(config: ConfigFile) -> Self {
        IanState {
            active_pet_id: config.app.active_pet,
            current_behavior: Default::default(),
            current_animation: "idle".to_string(),
            position: config.position,
            active_resource_pack: config.app.active_resource_pack,
            behavior_mode: config.behavior.mode,
        }
    }
}
