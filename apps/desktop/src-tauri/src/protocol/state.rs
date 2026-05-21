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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct IanState {
    pub active_pet_id: String,
    pub current_behavior: CurrentBehavior,
    pub current_animation: String,
    pub position: Position,
    pub active_resource_pack: String,
    pub behavior_mode: BehaviorMode,
}

impl Default for IanState {
    fn default() -> Self {
        Self {
            active_pet_id: "ian-kitten".to_string(),
            current_behavior: CurrentBehavior::Idle,
            current_animation: "idle".to_string(),
            position: Position { x: 0.0, y: 0.0 },
            active_resource_pack: "ian-kitten".to_string(),
            behavior_mode: BehaviorMode::Normal,
        }
    }
}
