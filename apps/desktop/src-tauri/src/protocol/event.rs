use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum BuildTestStatus {
    Success,
    Failure,
}

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
    MouseNear { x: f64, y: f64, now_ms: i64 },
    #[serde(rename = "mouse.leave")]
    MouseLeave { x: f64, y: f64 },
    #[serde(rename = "mouse.drag_start")]
    MouseDragStart { x: f64, y: f64 },
    #[serde(rename = "mouse.drag_end")]
    MouseDragEnd { x: f64, y: f64 },
    #[serde(rename = "dialogue.user_message")]
    DialogueUserMessage { text: String },
    #[serde(rename = "bubble.input_started")]
    BubbleInputStarted,
    #[serde(rename = "bubble.input_ended")]
    BubbleInputEnded,
    #[serde(rename = "developer.git_status_changed")]
    DeveloperGitStatusChanged {
        branch: String,
        dirty: bool,
        short_commit: String,
    },
    #[serde(rename = "developer.build_test_summary")]
    DeveloperBuildTestSummary {
        tool: String,
        status: BuildTestStatus,
        duration_ms: u64,
        tests_total: u32,
        tests_failed: u32,
        error_kind: Option<String>,
    },
    #[serde(rename = "keyboard.rhythm")]
    KeyboardRhythm {
        window_ms: u64,
        intensity: String,
        count: u32,
    },
    #[serde(rename = "active_app.presence")]
    ActiveAppPresence {
        category: String,
        confidence: f32,
        app_id: Option<String>,
    },
}

impl IanEvent {
    pub fn event_type(&self) -> &'static str {
        match self {
            Self::AppStarted => "app.started",
            Self::TimeTick { .. } => "time.tick",
            Self::MouseClick { .. } => "mouse.click",
            Self::MouseDoubleClick { .. } => "mouse.double_click",
            Self::MouseNear { .. } => "mouse.near",
            Self::MouseLeave { .. } => "mouse.leave",
            Self::MouseDragStart { .. } => "mouse.drag_start",
            Self::MouseDragEnd { .. } => "mouse.drag_end",
            Self::DialogueUserMessage { .. } => "dialogue.user_message",
            Self::BubbleInputStarted => "bubble.input_started",
            Self::BubbleInputEnded => "bubble.input_ended",
            Self::DeveloperGitStatusChanged { .. } => "developer.git_status_changed",
            Self::DeveloperBuildTestSummary { .. } => "developer.build_test_summary",
            Self::KeyboardRhythm { .. } => "keyboard.rhythm",
            Self::ActiveAppPresence { .. } => "active_app.presence",
        }
    }
}
