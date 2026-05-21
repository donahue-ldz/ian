use std::{fs, io, path::Path};

use directories::BaseDirs;
use serde::{Deserialize, Serialize};

use crate::protocol::{BehaviorMode, IanState, Position};

#[derive(Debug, Serialize, Deserialize)]
struct ConfigFile {
    app: AppConfig,
    behavior: BehaviorConfig,
    #[serde(default)]
    reminder: ReminderConfig,
    #[serde(default)]
    capabilities: CapabilityConfig,
    position: Position,
    #[serde(default)]
    home_anchor: Option<Position>,
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

#[derive(Debug, Serialize, Deserialize)]
struct ReminderConfig {
    enabled: bool,
}

impl Default for ReminderConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct CapabilityConfig {
    byom_enabled: bool,
    git_metadata_enabled: bool,
    build_test_events_enabled: bool,
    keyboard_rhythm_enabled: bool,
    active_app_presence_enabled: bool,
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

        assert_eq!(state.active_pet_id, "ian-alpaca");
        assert_eq!(state.position.x, 0.0);
        assert!(fs::read_to_string(dir.path().join("config.toml"))
            .expect("read recovered config")
            .contains("ian-alpaca"));
    }

    #[test]
    fn persist_state_round_trips_position_and_behavior_mode() {
        let dir = tempdir().expect("temp dir");
        let mut state = IanState::default();
        state.position = Position { x: 44.0, y: 88.0 };
        state.home_anchor = Position { x: 40.0, y: 80.0 };
        state.behavior_mode = BehaviorMode::Lively;
        state.reminders_enabled = false;

        persist_state(dir.path(), &state).expect("persist state");
        let loaded = load_state(dir.path()).expect("load state");

        assert_eq!(loaded.position.x, 44.0);
        assert_eq!(loaded.position.y, 88.0);
        assert_eq!(loaded.home_anchor.x, 40.0);
        assert_eq!(loaded.home_anchor.y, 80.0);
        assert!(matches!(loaded.behavior_mode, BehaviorMode::Lively));
        assert!(!loaded.reminders_enabled);
    }

    #[test]
    fn load_state_defaults_reminders_enabled_for_older_config() {
        let dir = tempdir().expect("temp dir");
        fs::write(
            dir.path().join("config.toml"),
            r#"
[app]
active_pet = "ian-alpaca"
active_resource_pack = "ian-alpaca"

[behavior]
mode = "normal"

[position]
x = 12.0
y = 24.0
"#,
        )
        .expect("write older config");

        let loaded = load_state(dir.path()).expect("load old config");

        assert!(loaded.reminders_enabled);
        assert_eq!(loaded.home_anchor.x, 12.0);
        assert_eq!(loaded.home_anchor.y, 24.0);
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
            reminder: ReminderConfig {
                enabled: state.reminders_enabled,
            },
            capabilities: CapabilityConfig {
                byom_enabled: state.byom_enabled,
                git_metadata_enabled: state.git_metadata_enabled,
                build_test_events_enabled: state.build_test_events_enabled,
                keyboard_rhythm_enabled: state.keyboard_rhythm_enabled,
                active_app_presence_enabled: state.active_app_presence_enabled,
            },
            position: state.position,
            home_anchor: Some(state.home_anchor),
        }
    }
}

impl From<ConfigFile> for IanState {
    fn from(config: ConfigFile) -> Self {
        IanState {
            active_pet_id: config.app.active_pet,
            current_behavior: Default::default(),
            current_animation: "idle".to_string(),
            position: config.position.clone(),
            active_resource_pack: config.app.active_resource_pack,
            behavior_mode: config.behavior.mode,
            reminders_enabled: config.reminder.enabled,
            byom_enabled: config.capabilities.byom_enabled,
            git_metadata_enabled: config.capabilities.git_metadata_enabled,
            build_test_events_enabled: config.capabilities.build_test_events_enabled,
            keyboard_rhythm_enabled: config.capabilities.keyboard_rhythm_enabled,
            active_app_presence_enabled: config.capabilities.active_app_presence_enabled,
            home_anchor: config.home_anchor.unwrap_or(config.position),
        }
    }
}
