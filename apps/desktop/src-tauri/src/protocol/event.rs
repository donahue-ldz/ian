use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(tag = "type")]
#[ts(export)]
pub enum IanEvent {
    #[serde(rename = "app.started")]
    AppStarted,
    #[serde(rename = "time.tick")]
    TimeTick { now_ms: i64 },
    #[serde(rename = "mouse.click")]
    MouseClick { x: f64, y: f64 },
    #[serde(rename = "mouse.double_click")]
    MouseDoubleClick { x: f64, y: f64 },
    #[serde(rename = "mouse.near")]
    MouseNear { x: f64, y: f64 },
    #[serde(rename = "mouse.drag_start")]
    MouseDragStart { x: f64, y: f64 },
    #[serde(rename = "mouse.drag_end")]
    MouseDragEnd { x: f64, y: f64 },
    #[serde(rename = "dialogue.user_message")]
    DialogueUserMessage { text: String },
}

impl IanEvent {
    pub fn event_type(&self) -> &'static str {
        match self {
            Self::AppStarted => "app.started",
            Self::TimeTick { .. } => "time.tick",
            Self::MouseClick { .. } => "mouse.click",
            Self::MouseDoubleClick { .. } => "mouse.double_click",
            Self::MouseNear { .. } => "mouse.near",
            Self::MouseDragStart { .. } => "mouse.drag_start",
            Self::MouseDragEnd { .. } => "mouse.drag_end",
            Self::DialogueUserMessage { .. } => "dialogue.user_message",
        }
    }
}
