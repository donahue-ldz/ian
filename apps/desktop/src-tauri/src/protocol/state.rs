use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum CurrentBehavior {
    Idle,
    Walking,
    Happy,
    Running,
    Sleeping,
}

impl Default for CurrentBehavior {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum BehaviorMode {
    Quiet,
    Normal,
    Lively,
}

impl Default for BehaviorMode {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct QuietHours {
    pub enabled: bool,
    pub start_minute: u16,
    pub end_minute: u16,
}

impl Default for QuietHours {
    fn default() -> Self {
        Self {
            enabled: false,
            start_minute: 22 * 60,
            end_minute: 7 * 60,
        }
    }
}

impl QuietHours {
    pub fn is_active_at_minute(&self, minute: u16) -> bool {
        if !self.enabled {
            return false;
        }

        let start = self.start_minute.min(24 * 60);
        let end = self.end_minute.min(24 * 60);
        let minute = minute.min(24 * 60);

        if start == end {
            return true;
        }

        if start < end {
            minute >= start && minute < end
        } else {
            minute >= start || minute < end
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct DeveloperWorkspace {
    pub bound: bool,
    pub workspace_id: Option<String>,
    pub display_name: Option<String>,
    pub root_path: Option<String>,
    pub enabled: bool,
}

impl Default for DeveloperWorkspace {
    fn default() -> Self {
        Self {
            bound: false,
            workspace_id: None,
            display_name: None,
            root_path: None,
            enabled: false,
        }
    }
}

impl DeveloperWorkspace {
    pub fn is_active(&self) -> bool {
        self.bound && self.enabled && self.workspace_id.is_some()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct DeveloperSnooze {
    pub enabled: bool,
    pub until_ms: Option<i64>,
    pub reason: Option<String>,
}

impl Default for DeveloperSnooze {
    fn default() -> Self {
        Self {
            enabled: false,
            until_ms: None,
            reason: None,
        }
    }
}

impl DeveloperSnooze {
    pub fn is_active_at(&self, now_ms: i64) -> bool {
        if !self.enabled {
            return false;
        }

        self.until_ms.map(|until| now_ms < until).unwrap_or(true)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct IanState {
    pub active_pet_id: String,
    pub current_behavior: CurrentBehavior,
    pub current_animation: String,
    pub position: Position,
    pub active_resource_pack: String,
    pub behavior_mode: BehaviorMode,
    pub reminders_enabled: bool,
    pub byom_enabled: bool,
    pub git_metadata_enabled: bool,
    pub build_test_events_enabled: bool,
    pub keyboard_rhythm_enabled: bool,
    pub active_app_presence_enabled: bool,
    pub home_anchor: Position,
    pub quiet_hours: QuietHours,
    pub movement_intensity: String,
    pub bubble_frequency: String,
    pub rest_behavior: String,
    pub surface_scale: f64,
    pub diagnostics_enabled: bool,
    pub day_phase: String,
    pub is_dragging: bool,
    pub is_bubble_input_active: bool,
    pub developer_workspace: DeveloperWorkspace,
    pub developer_snooze: DeveloperSnooze,
    pub active_app_category: Option<String>,
}

impl Default for IanState {
    fn default() -> Self {
        Self {
            active_pet_id: "ian-alpaca".to_string(),
            current_behavior: CurrentBehavior::Idle,
            current_animation: "idle".to_string(),
            position: Position { x: 0.0, y: 0.0 },
            active_resource_pack: "ian-alpaca".to_string(),
            behavior_mode: BehaviorMode::Normal,
            reminders_enabled: true,
            byom_enabled: false,
            git_metadata_enabled: false,
            build_test_events_enabled: false,
            keyboard_rhythm_enabled: false,
            active_app_presence_enabled: false,
            home_anchor: Position { x: 0.0, y: 0.0 },
            quiet_hours: QuietHours::default(),
            movement_intensity: "normal".to_string(),
            bubble_frequency: "normal".to_string(),
            rest_behavior: "normal".to_string(),
            surface_scale: 1.0,
            diagnostics_enabled: true,
            day_phase: "day".to_string(),
            is_dragging: false,
            is_bubble_input_active: false,
            developer_workspace: DeveloperWorkspace::default(),
            developer_snooze: DeveloperSnooze::default(),
            active_app_category: None,
        }
    }
}
