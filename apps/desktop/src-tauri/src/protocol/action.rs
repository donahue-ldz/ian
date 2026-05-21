use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::state::IanState;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum MovementSpeed {
    Slow,
    Normal,
    Fast,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(tag = "type")]
#[ts(export)]
pub enum IanAction {
    #[serde(rename = "animation.play")]
    AnimationPlay { name: String, looped: bool },
    #[serde(rename = "movement.move_to")]
    MovementMoveTo {
        x: f64,
        y: f64,
        speed: MovementSpeed,
    },
    #[serde(rename = "speech.show")]
    SpeechShow {
        text: String,
        mood: Option<String>,
        duration_ms: Option<u64>,
    },
    #[serde(rename = "bubble.open")]
    BubbleOpen,
    #[serde(rename = "bubble.close")]
    BubbleClose,
    #[serde(rename = "behavior.run_around")]
    BehaviorRunAround { duration_ms: u64 },
    #[serde(rename = "state.sync")]
    StateSync { state: IanState },
}
