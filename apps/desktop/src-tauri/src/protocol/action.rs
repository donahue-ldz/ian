use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::state::{IanState, PlayfulDiagnostic, PlayfulState};

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum MovementSpeed {
    Slow,
    Normal,
    Fast,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum MotionProfile {
    #[default]
    Gentle,
    Playful,
    Settle,
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
        #[serde(default)]
        profile: MotionProfile,
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
    #[serde(rename = "behavior.zoomies")]
    BehaviorZoomies { duration_ms: u64, reason: String },
    #[serde(rename = "effect.play")]
    EffectPlay {
        name: String,
        intensity: String,
        duration_ms: u64,
    },
    #[serde(rename = "appearance.scale_to")]
    AppearanceScaleTo { scale: f64, duration_ms: u64 },
    #[serde(rename = "playful.state")]
    PlayfulStateSet {
        state: PlayfulState,
        until_ms: Option<i64>,
    },
    #[serde(rename = "playful.diagnostic")]
    PlayfulDiagnostic {
        timestamp_ms: i64,
        reason: String,
        result: String,
        cooldown_key: Option<String>,
        chosen_reaction_key: Option<String>,
    },
    #[serde(rename = "state.sync")]
    StateSync { state: IanState },
}

impl From<PlayfulDiagnostic> for IanAction {
    fn from(diagnostic: PlayfulDiagnostic) -> Self {
        Self::PlayfulDiagnostic {
            timestamp_ms: diagnostic.timestamp_ms,
            reason: diagnostic.reason,
            result: diagnostic.result,
            cooldown_key: diagnostic.cooldown_key,
            chosen_reaction_key: diagnostic.chosen_reaction_key,
        }
    }
}
