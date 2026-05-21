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
    #[serde(rename = "mouse.drag_start")]
    MouseDragStart { x: f64, y: f64 },
    #[serde(rename = "mouse.drag_end")]
    MouseDragEnd { x: f64, y: f64 },
    #[serde(rename = "dialogue.user_message")]
    DialogueUserMessage { text: String },
}
