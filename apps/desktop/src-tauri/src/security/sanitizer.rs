use crate::protocol::{IanError, IanEvent};

#[derive(Default)]
pub struct Sanitizer;

impl Sanitizer {
    pub fn inspect(&self, event: &IanEvent) -> Result<(), IanError> {
        match event {
            IanEvent::DialogueUserMessage { text } => self.inspect_text(text, 256, "dialogue"),
            IanEvent::DeveloperGitStatusChanged {
                workspace_id,
                branch,
                short_commit,
                ..
            } => {
                if let Some(workspace_id) = workspace_id {
                    self.inspect_workspace_id(workspace_id)?;
                }
                self.inspect_text(branch, 80, "git_branch")?;
                self.inspect_short_hash(short_commit)
            }
            IanEvent::DeveloperBuildTestSummary {
                workspace_id,
                tool,
                error_kind,
                ..
            } => {
                if let Some(workspace_id) = workspace_id {
                    self.inspect_workspace_id(workspace_id)?;
                }
                self.inspect_text(tool, 40, "build_tool")?;
                if let Some(error_kind) = error_kind {
                    self.inspect_error_kind(error_kind)?;
                }
                Ok(())
            }
            IanEvent::KeyboardRhythm { intensity, .. } => {
                if intensity.contains("key")
                    || intensity.contains("text")
                    || intensity.contains("shortcut")
                {
                    return Err(IanError::new(
                        "keyboard_payload_contains_content",
                        "Keyboard rhythm payload must not include key or text content.",
                    ));
                }
                self.inspect_text(intensity, 24, "keyboard_intensity")
            }
            IanEvent::ActiveAppPresence {
                category, app_id, ..
            } => {
                self.inspect_text(category, 32, "active_app_category")?;
                if let Some(app_id) = app_id {
                    self.inspect_text(app_id, 80, "active_app_id")?;
                }
                Ok(())
            }
            IanEvent::SystemShortcutTriggered { action, .. } => {
                if action != "find_ian" {
                    return Err(IanError::new(
                        "shortcut_action_rejected",
                        "System shortcut action is not allowlisted.",
                    ));
                }
                Ok(())
            }
            IanEvent::ScreenBounds {
                x,
                y,
                width,
                height,
            } => {
                if !x.is_finite()
                    || !y.is_finite()
                    || !width.is_finite()
                    || !height.is_finite()
                    || *width <= 0.0
                    || *height <= 0.0
                {
                    return Err(IanError::new(
                        "screen_bounds_payload_rejected",
                        "Screen bounds must be finite and positive.",
                    ));
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn inspect_text(&self, text: &str, max_chars: usize, label: &str) -> Result<(), IanError> {
        if text.chars().count() > max_chars {
            return Err(IanError::new(
                format!("{label}_payload_too_large"),
                "Event payload is too large.",
            ));
        }

        if text.contains('<') || text.contains('>') || text.contains('\0') {
            return Err(IanError::new(
                format!("{label}_payload_rejected"),
                "Event payload contains unsupported characters.",
            ));
        }

        Ok(())
    }

    fn inspect_workspace_id(&self, text: &str) -> Result<(), IanError> {
        self.inspect_text(text, 80, "workspace_id")?;
        if text.contains('/') || text.contains('\\') || text.starts_with('~') {
            return Err(IanError::new(
                "workspace_id_payload_rejected",
                "Workspace payload must be an opaque local id, not a filesystem path.",
            ));
        }
        Ok(())
    }

    fn inspect_error_kind(&self, text: &str) -> Result<(), IanError> {
        self.inspect_text(text, 80, "build_error_kind")?;
        if text.contains('{') || text.contains('}') || text.contains(';') || text.contains("fn ") {
            return Err(IanError::new(
                "build_error_kind_payload_rejected",
                "Build error payload must be a short category, not source content.",
            ));
        }
        Ok(())
    }

    fn inspect_short_hash(&self, value: &str) -> Result<(), IanError> {
        if value.len() > 12 || !value.chars().all(|ch| ch.is_ascii_hexdigit()) {
            return Err(IanError::new(
                "git_commit_payload_rejected",
                "Git commit payload must be a short hash.",
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Sanitizer;
    use crate::protocol::{BuildTestStatus, IanEvent};

    #[test]
    fn rejects_long_dialogue_and_basic_dangerous_text() {
        let sanitizer = Sanitizer::default();

        assert!(sanitizer
            .inspect(&IanEvent::DialogueUserMessage {
                text: "a".repeat(257),
            })
            .is_err());
        assert!(sanitizer
            .inspect(&IanEvent::DialogueUserMessage {
                text: "<script>".to_string(),
            })
            .is_err());
    }

    #[test]
    fn rejects_sensitive_developer_payload_shapes() {
        let sanitizer = Sanitizer::default();

        assert!(sanitizer
            .inspect(&IanEvent::DeveloperBuildTestSummary {
                workspace_id: Some("workspace-1".to_string()),
                tool: "cargo".to_string(),
                status: BuildTestStatus::Failure,
                duration_ms: 10,
                tests_total: 1,
                tests_failed: 1,
                error_kind: Some("x".repeat(81)),
            })
            .is_err());
        assert!(sanitizer
            .inspect(&IanEvent::KeyboardRhythm {
                window_ms: 60_000,
                intensity: "key:a".to_string(),
                count: 10,
            })
            .is_err());
    }

    #[test]
    fn rejects_code_content_paths_and_payloads_over_budget() {
        let sanitizer = Sanitizer::default();

        assert!(sanitizer
            .inspect(&IanEvent::DeveloperGitStatusChanged {
                workspace_id: Some("/Users/alice/private/repo".to_string()),
                branch: "main".to_string(),
                dirty: false,
                short_commit: "abc1234".to_string(),
            })
            .is_err());
        assert!(sanitizer
            .inspect(&IanEvent::DeveloperBuildTestSummary {
                workspace_id: Some("workspace-1".to_string()),
                tool: "cargo".to_string(),
                status: BuildTestStatus::Failure,
                duration_ms: 10,
                tests_total: 1,
                tests_failed: 1,
                error_kind: Some("fn main() { panic!(); }".to_string()),
            })
            .is_err());
        assert!(sanitizer
            .inspect(&IanEvent::DialogueUserMessage {
                text: "x".repeat(257),
            })
            .is_err());
    }
}
