use std::{fs, io, path::Path};

use directories::BaseDirs;
use serde::{Deserialize, Serialize};

use crate::protocol::{
    BehaviorMode, DeveloperSnooze, DeveloperWorkspace, IanState, PlayfulEnergy, Position,
    QuietHours,
};

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
    #[serde(default)]
    quiet_hours: QuietHours,
    #[serde(default)]
    creature: CreatureConfig,
    #[serde(default)]
    developer_workspace: DeveloperWorkspace,
    #[serde(default)]
    developer_snooze: DeveloperSnooze,
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
struct CreatureConfig {
    movement_intensity: String,
    bubble_frequency: String,
    rest_behavior: String,
    #[serde(default)]
    playful_energy: PlayfulEnergy,
    #[serde(default)]
    playful_snoozed_until_ms: Option<i64>,
    surface_scale: f64,
    diagnostics_enabled: bool,
}

impl Default for CreatureConfig {
    fn default() -> Self {
        Self {
            movement_intensity: "normal".to_string(),
            bubble_frequency: "normal".to_string(),
            rest_behavior: "normal".to_string(),
            playful_energy: PlayfulEnergy::Normal,
            playful_snoozed_until_ms: None,
            surface_scale: 1.0,
            diagnostics_enabled: true,
        }
    }
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
    use crate::protocol::{
        BehaviorMode, DeveloperSnooze, DeveloperWorkspace, IanState, PlayfulEnergy, Position,
    };

    #[test]
    fn load_state_recovers_corrupt_config_to_default() {
        let dir = tempdir().expect("temp dir");
        fs::write(dir.path().join("config.toml"), "not = [valid").expect("write corrupt config");

        let state = load_state(dir.path()).expect("load recovered state");

        assert_eq!(state.active_pet_id, "ian-puppy");
        assert_eq!(state.active_resource_pack, "ian-puppy");
        assert_eq!(state.position.x, 0.0);
        assert!(fs::read_to_string(dir.path().join("config.toml"))
            .expect("read recovered config")
            .contains("ian-puppy"));
    }

    #[test]
    fn load_state_uses_puppy_for_new_default_config() {
        let dir = tempdir().expect("temp dir");

        let loaded = load_state(dir.path()).expect("load default config");

        assert_eq!(loaded.active_pet_id, "ian-puppy");
        assert_eq!(loaded.active_resource_pack, "ian-puppy");
    }

    #[test]
    fn persist_state_round_trips_position_and_behavior_mode() {
        let dir = tempdir().expect("temp dir");
        let mut state = IanState::default();
        state.position = Position { x: 44.0, y: 88.0 };
        state.home_anchor = Position { x: 40.0, y: 80.0 };
        state.behavior_mode = BehaviorMode::Lively;
        state.reminders_enabled = false;
        state.quiet_hours.enabled = true;
        state.quiet_hours.start_minute = 22 * 60;
        state.quiet_hours.end_minute = 7 * 60;
        state.movement_intensity = "low".to_string();
        state.bubble_frequency = "quiet".to_string();
        state.rest_behavior = "restful".to_string();
        state.surface_scale = 1.2;
        state.diagnostics_enabled = false;
        state.playful_energy = PlayfulEnergy::High;
        state.git_metadata_enabled = true;
        state.build_test_events_enabled = true;
        state.keyboard_rhythm_enabled = false;
        state.active_app_presence_enabled = true;
        state.developer_workspace = DeveloperWorkspace {
            bound: true,
            workspace_id: Some("workspace-1".to_string()),
            display_name: Some("Ian".to_string()),
            root_path: Some("~/Git/ian".to_string()),
            enabled: true,
        };
        state.developer_snooze = DeveloperSnooze {
            enabled: true,
            until_ms: Some(2_000),
            reason: Some("manual".to_string()),
        };

        persist_state(dir.path(), &state).expect("persist state");
        let loaded = load_state(dir.path()).expect("load state");

        assert_eq!(loaded.position.x, 44.0);
        assert_eq!(loaded.position.y, 88.0);
        assert_eq!(loaded.home_anchor.x, 40.0);
        assert_eq!(loaded.home_anchor.y, 80.0);
        assert!(matches!(loaded.behavior_mode, BehaviorMode::Lively));
        assert!(!loaded.reminders_enabled);
        assert!(loaded.quiet_hours.enabled);
        assert_eq!(loaded.quiet_hours.start_minute, 22 * 60);
        assert_eq!(loaded.quiet_hours.end_minute, 7 * 60);
        assert_eq!(loaded.movement_intensity, "low");
        assert_eq!(loaded.bubble_frequency, "quiet");
        assert_eq!(loaded.rest_behavior, "restful");
        assert_eq!(loaded.surface_scale, 1.2);
        assert!(!loaded.diagnostics_enabled);
        assert!(matches!(loaded.playful_energy, PlayfulEnergy::High));
        assert!(loaded.git_metadata_enabled);
        assert!(loaded.build_test_events_enabled);
        assert!(!loaded.keyboard_rhythm_enabled);
        assert!(loaded.active_app_presence_enabled);
        assert!(loaded.developer_workspace.bound);
        assert_eq!(
            loaded.developer_workspace.workspace_id.as_deref(),
            Some("workspace-1")
        );
        assert_eq!(
            loaded.developer_workspace.display_name.as_deref(),
            Some("Ian")
        );
        assert_eq!(
            loaded.developer_workspace.root_path.as_deref(),
            Some("~/Git/ian")
        );
        assert!(loaded.developer_workspace.enabled);
        assert!(loaded.developer_snooze.enabled);
        assert_eq!(loaded.developer_snooze.until_ms, Some(2_000));
    }

    #[test]
    fn quiet_hours_support_cross_midnight_and_disabled_state() {
        let mut state = IanState::default();
        state.quiet_hours.enabled = true;
        state.quiet_hours.start_minute = 22 * 60;
        state.quiet_hours.end_minute = 7 * 60;

        assert!(state.quiet_hours.is_active_at_minute(23 * 60));
        assert!(state.quiet_hours.is_active_at_minute(6 * 60 + 30));
        assert!(!state.quiet_hours.is_active_at_minute(12 * 60));

        state.quiet_hours.enabled = false;
        assert!(!state.quiet_hours.is_active_at_minute(23 * 60));
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

        assert_eq!(loaded.active_pet_id, "ian-alpaca");
        assert_eq!(loaded.active_resource_pack, "ian-alpaca");
        assert!(loaded.reminders_enabled);
        assert_eq!(loaded.home_anchor.x, 12.0);
        assert_eq!(loaded.home_anchor.y, 24.0);
        assert!(!loaded.git_metadata_enabled);
        assert!(!loaded.build_test_events_enabled);
        assert!(!loaded.keyboard_rhythm_enabled);
        assert!(!loaded.active_app_presence_enabled);
        assert!(!loaded.developer_workspace.bound);
        assert!(!loaded.developer_snooze.enabled);
        assert!(matches!(loaded.playful_energy, PlayfulEnergy::Normal));
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
            quiet_hours: state.quiet_hours,
            creature: CreatureConfig {
                movement_intensity: state.movement_intensity,
                bubble_frequency: state.bubble_frequency,
                rest_behavior: state.rest_behavior,
                playful_energy: state.playful_energy,
                playful_snoozed_until_ms: state.playful_snoozed_until_ms,
                surface_scale: state.surface_scale,
                diagnostics_enabled: state.diagnostics_enabled,
            },
            developer_workspace: state.developer_workspace,
            developer_snooze: state.developer_snooze,
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
            screen_bounds: None,
            last_user_interaction_ms: 0,
            quiet_hours: config.quiet_hours,
            movement_intensity: config.creature.movement_intensity,
            bubble_frequency: config.creature.bubble_frequency,
            rest_behavior: config.creature.rest_behavior,
            playful_energy: config.creature.playful_energy,
            playful_state: Default::default(),
            playful_state_until_ms: None,
            playful_snoozed_until_ms: config.creature.playful_snoozed_until_ms,
            last_playful_diagnostic: None,
            surface_scale: config.creature.surface_scale,
            diagnostics_enabled: config.creature.diagnostics_enabled,
            day_phase: "day".to_string(),
            is_dragging: false,
            is_bubble_input_active: false,
            developer_workspace: config.developer_workspace,
            developer_snooze: config.developer_snooze,
            active_app_category: None,
        }
    }
}
