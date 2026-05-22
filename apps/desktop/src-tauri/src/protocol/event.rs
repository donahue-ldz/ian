use serde::{Deserialize, Deserializer, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum BuildTestStatus {
    Success,
    Failure,
}

#[derive(Debug, Clone, Serialize, TS)]
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
    #[serde(rename = "mouse.chase_candidate")]
    MouseChaseCandidate { x: f64, y: f64, now_ms: i64 },
    #[serde(rename = "screen.bounds")]
    ScreenBounds {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },
    #[serde(rename = "dialogue.user_message")]
    DialogueUserMessage { text: String },
    #[serde(rename = "bubble.input_started")]
    BubbleInputStarted,
    #[serde(rename = "bubble.input_ended")]
    BubbleInputEnded,
    #[serde(rename = "developer.git_status_changed")]
    DeveloperGitStatusChanged {
        workspace_id: Option<String>,
        branch: String,
        dirty: bool,
        short_commit: String,
    },
    #[serde(rename = "developer.build_test_summary")]
    DeveloperBuildTestSummary {
        workspace_id: Option<String>,
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
    #[serde(rename = "system.shortcut_triggered")]
    SystemShortcutTriggered { action: String, now_ms: i64 },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
enum IanEventWire {
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
    #[serde(rename = "mouse.chase_candidate")]
    MouseChaseCandidate { x: f64, y: f64, now_ms: i64 },
    #[serde(rename = "screen.bounds")]
    ScreenBounds {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },
    #[serde(rename = "dialogue.user_message")]
    DialogueUserMessage { text: String },
    #[serde(rename = "bubble.input_started")]
    BubbleInputStarted,
    #[serde(rename = "bubble.input_ended")]
    BubbleInputEnded,
    #[serde(rename = "developer.git_status_changed")]
    DeveloperGitStatusChanged {
        workspace_id: Option<String>,
        branch: String,
        dirty: bool,
        short_commit: String,
    },
    #[serde(rename = "developer.build_test_summary")]
    DeveloperBuildTestSummary {
        workspace_id: Option<String>,
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
    #[serde(rename = "system.shortcut_triggered")]
    SystemShortcutTriggered { action: String, now_ms: i64 },
}

impl<'de> Deserialize<'de> for IanEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(IanEventWire::deserialize(deserializer)?.into())
    }
}

impl From<IanEventWire> for IanEvent {
    fn from(event: IanEventWire) -> Self {
        match event {
            IanEventWire::AppStarted => Self::AppStarted,
            IanEventWire::TimeTick { now_ms } => Self::TimeTick { now_ms },
            IanEventWire::MouseClick { x, y } => Self::MouseClick { x, y },
            IanEventWire::MouseDoubleClick { x, y } => Self::MouseDoubleClick { x, y },
            IanEventWire::MouseNear { x, y, now_ms } => Self::MouseNear { x, y, now_ms },
            IanEventWire::MouseLeave { x, y } => Self::MouseLeave { x, y },
            IanEventWire::MouseDragStart { x, y } => Self::MouseDragStart { x, y },
            IanEventWire::MouseDragEnd { x, y } => Self::MouseDragEnd { x, y },
            IanEventWire::MouseChaseCandidate { x, y, now_ms } => {
                Self::MouseChaseCandidate { x, y, now_ms }
            }
            IanEventWire::ScreenBounds {
                x,
                y,
                width,
                height,
            } => Self::ScreenBounds {
                x,
                y,
                width,
                height,
            },
            IanEventWire::DialogueUserMessage { text } => Self::DialogueUserMessage { text },
            IanEventWire::BubbleInputStarted => Self::BubbleInputStarted,
            IanEventWire::BubbleInputEnded => Self::BubbleInputEnded,
            IanEventWire::DeveloperGitStatusChanged {
                workspace_id,
                branch,
                dirty,
                short_commit,
            } => Self::DeveloperGitStatusChanged {
                workspace_id,
                branch,
                dirty,
                short_commit,
            },
            IanEventWire::DeveloperBuildTestSummary {
                workspace_id,
                tool,
                status,
                duration_ms,
                tests_total,
                tests_failed,
                error_kind,
            } => Self::DeveloperBuildTestSummary {
                workspace_id,
                tool,
                status,
                duration_ms,
                tests_total,
                tests_failed,
                error_kind,
            },
            IanEventWire::KeyboardRhythm {
                window_ms,
                intensity,
                count,
            } => Self::KeyboardRhythm {
                window_ms,
                intensity,
                count,
            },
            IanEventWire::ActiveAppPresence {
                category,
                confidence,
                app_id,
            } => Self::ActiveAppPresence {
                category,
                confidence,
                app_id,
            },
            IanEventWire::SystemShortcutTriggered { action, now_ms } => {
                Self::SystemShortcutTriggered { action, now_ms }
            }
        }
    }
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
            Self::MouseChaseCandidate { .. } => "mouse.chase_candidate",
            Self::ScreenBounds { .. } => "screen.bounds",
            Self::DialogueUserMessage { .. } => "dialogue.user_message",
            Self::BubbleInputStarted => "bubble.input_started",
            Self::BubbleInputEnded => "bubble.input_ended",
            Self::DeveloperGitStatusChanged { .. } => "developer.git_status_changed",
            Self::DeveloperBuildTestSummary { .. } => "developer.build_test_summary",
            Self::KeyboardRhythm { .. } => "keyboard.rhythm",
            Self::ActiveAppPresence { .. } => "active_app.presence",
            Self::SystemShortcutTriggered { .. } => "system.shortcut_triggered",
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::IanEvent;

    #[test]
    fn developer_events_reject_sensitive_extra_fields() {
        let keyboard_payload = json!({
            "type": "keyboard.rhythm",
            "window_ms": 60_000,
            "intensity": "active",
            "count": 20,
            "key": "a"
        });
        let build_payload = json!({
            "type": "developer.build_test_summary",
            "workspace_id": "workspace-1",
            "tool": "cargo",
            "status": "failure",
            "duration_ms": 1200,
            "tests_total": 3,
            "tests_failed": 1,
            "stdout": "full terminal output"
        });
        let app_payload = json!({
            "type": "active_app.presence",
            "category": "editor",
            "confidence": 0.8,
            "window_title": "private file name"
        });

        assert!(serde_json::from_value::<IanEvent>(keyboard_payload).is_err());
        assert!(serde_json::from_value::<IanEvent>(build_payload).is_err());
        assert!(serde_json::from_value::<IanEvent>(app_payload).is_err());
    }

    #[test]
    fn mouse_chase_candidate_accepts_only_low_sensitive_coordinates() {
        let payload = json!({
            "type": "mouse.chase_candidate",
            "x": 640.0,
            "y": 420.0,
            "now_ms": 300_000
        });
        let sensitive_payload = json!({
            "type": "mouse.chase_candidate",
            "x": 640.0,
            "y": 420.0,
            "now_ms": 300_000,
            "window_title": "private doc"
        });

        assert!(serde_json::from_value::<IanEvent>(payload).is_ok());
        assert!(serde_json::from_value::<IanEvent>(sensitive_payload).is_err());
    }

    #[test]
    fn find_ian_shortcut_accepts_only_action_and_time() {
        let payload = json!({
            "type": "system.shortcut_triggered",
            "action": "find_ian",
            "now_ms": 300_000
        });
        let sensitive_payload = json!({
            "type": "system.shortcut_triggered",
            "action": "find_ian",
            "now_ms": 300_000,
            "text": "typed input"
        });

        assert!(serde_json::from_value::<IanEvent>(payload).is_ok());
        assert!(serde_json::from_value::<IanEvent>(sensitive_payload).is_err());
    }
}
