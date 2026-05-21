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
        ensure_default_config(app_dir)?;
        return Ok(IanState::default());
    }

    let content = fs::read_to_string(config_path)?;
    let config: ConfigFile = match toml::from_str(&content) {
        Ok(config) => config,
        Err(_) => {
            let default_state = IanState::default();
            persist_state(app_dir, &default_state)?;
            return Ok(default_state);
        }
    };

    Ok(config.into())
}

pub fn persist_state(app_dir: &Path, state: &IanState) -> io::Result<()> {
    let config_path = app_dir.join("config.toml");
    let serialized = toml::to_string_pretty(&ConfigFile::from(state.clone()))
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    fs::write(config_path, serialized)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::{load_state, persist_state};
    use crate::protocol::{BehaviorMode, IanState, Position};

    #[test]
    fn load_state_recovers_corrupt_config_to_default() {
        let dir = tempdir().expect("temp dir");
        fs::write(dir.path().join("config.toml"), "not = [valid").expect("write corrupt config");

        let state = load_state(dir.path()).expect("load recovered state");

        assert_eq!(state.active_pet_id, "ian-kitten");
        assert_eq!(state.position.x, 0.0);
        assert!(fs::read_to_string(dir.path().join("config.toml"))
            .expect("read recovered config")
            .contains("ian-kitten"));
    }

    #[test]
    fn persist_state_round_trips_position_and_behavior_mode() {
        let dir = tempdir().expect("temp dir");
        let mut state = IanState::default();
        state.position = Position { x: 44.0, y: 88.0 };
        state.behavior_mode = BehaviorMode::Lively;

        persist_state(dir.path(), &state).expect("persist state");
        let loaded = load_state(dir.path()).expect("load state");

        assert_eq!(loaded.position.x, 44.0);
        assert_eq!(loaded.position.y, 88.0);
        assert!(matches!(loaded.behavior_mode, BehaviorMode::Lively));
    }
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
