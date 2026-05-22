use crate::protocol::IanState;
use crate::protocol::{IanError, IanEvent};

#[derive(Debug, Clone, Copy)]
pub struct PermissionState {
    pub byom_enabled: bool,
    pub git_metadata_enabled: bool,
    pub build_test_events_enabled: bool,
    pub keyboard_rhythm_enabled: bool,
    pub active_app_presence_enabled: bool,
    pub developer_workspace_bound: bool,
}

impl Default for PermissionState {
    fn default() -> Self {
        Self {
            byom_enabled: false,
            git_metadata_enabled: false,
            build_test_events_enabled: false,
            keyboard_rhythm_enabled: false,
            active_app_presence_enabled: false,
            developer_workspace_bound: false,
        }
    }
}

impl From<&IanState> for PermissionState {
    fn from(state: &IanState) -> Self {
        Self {
            byom_enabled: state.byom_enabled,
            git_metadata_enabled: state.git_metadata_enabled,
            build_test_events_enabled: state.build_test_events_enabled,
            keyboard_rhythm_enabled: state.keyboard_rhythm_enabled,
            active_app_presence_enabled: state.active_app_presence_enabled,
            developer_workspace_bound: state.developer_workspace.is_active(),
        }
    }
}

#[derive(Default)]
pub struct PermissionGate {
    state: PermissionState,
}

impl PermissionGate {
    pub fn set_state(&mut self, state: PermissionState) {
        self.state = state;
    }

    pub fn allow(&self, event: &IanEvent) -> Result<(), IanError> {
        match event {
            IanEvent::AppStarted
            | IanEvent::TimeTick { .. }
            | IanEvent::MouseClick { .. }
            | IanEvent::MouseDoubleClick { .. }
            | IanEvent::MouseNear { .. }
            | IanEvent::MouseLeave { .. }
            | IanEvent::MouseDragStart { .. }
            | IanEvent::MouseDragEnd { .. }
            | IanEvent::MouseChaseCandidate { .. }
            | IanEvent::ScreenBounds { .. }
            | IanEvent::SystemShortcutTriggered { .. }
            | IanEvent::DialogueUserMessage { .. }
            | IanEvent::BubbleInputStarted
            | IanEvent::BubbleInputEnded => Ok(()),
            IanEvent::DeveloperGitStatusChanged { workspace_id, .. }
                if self.state.git_metadata_enabled
                    && self.state.developer_workspace_bound
                    && workspace_id.is_some() =>
            {
                Ok(())
            }
            IanEvent::DeveloperBuildTestSummary { workspace_id, .. }
                if self.state.build_test_events_enabled
                    && self.state.developer_workspace_bound
                    && workspace_id.is_some() =>
            {
                Ok(())
            }
            IanEvent::KeyboardRhythm { .. } if self.state.keyboard_rhythm_enabled => Ok(()),
            IanEvent::ActiveAppPresence { .. } if self.state.active_app_presence_enabled => Ok(()),
            _ => Err(IanError::new(
                "permission_denied",
                "This Ian event source is not enabled.",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PermissionGate, PermissionState};
    use crate::protocol::{BuildTestStatus, IanEvent};

    #[test]
    fn developer_sources_are_denied_by_default() {
        let gate = PermissionGate::default();

        assert!(gate
            .allow(&IanEvent::DeveloperGitStatusChanged {
                workspace_id: Some("workspace-1".to_string()),
                branch: "main".to_string(),
                dirty: false,
                short_commit: "abc1234".to_string(),
            })
            .is_err());
        assert!(gate
            .allow(&IanEvent::KeyboardRhythm {
                window_ms: 60_000,
                intensity: "active".to_string(),
                count: 120,
            })
            .is_err());
    }

    #[test]
    fn enabled_low_sensitive_sources_are_allowed() {
        let mut gate = PermissionGate::default();
        gate.set_state(PermissionState {
            git_metadata_enabled: true,
            build_test_events_enabled: true,
            developer_workspace_bound: true,
            ..PermissionState::default()
        });

        assert!(gate
            .allow(&IanEvent::DeveloperGitStatusChanged {
                workspace_id: Some("workspace-1".to_string()),
                branch: "main".to_string(),
                dirty: false,
                short_commit: "abc1234".to_string(),
            })
            .is_ok());
        assert!(gate
            .allow(&IanEvent::DeveloperBuildTestSummary {
                workspace_id: Some("workspace-1".to_string()),
                tool: "cargo".to_string(),
                status: BuildTestStatus::Success,
                duration_ms: 1200,
                tests_total: 3,
                tests_failed: 0,
                error_kind: None,
            })
            .is_ok());
    }

    #[test]
    fn git_and_build_events_require_bound_workspace() {
        let git_event = IanEvent::DeveloperGitStatusChanged {
            workspace_id: Some("workspace-1".to_string()),
            branch: "main".to_string(),
            dirty: false,
            short_commit: "abc1234".to_string(),
        };
        let build_event = IanEvent::DeveloperBuildTestSummary {
            workspace_id: Some("workspace-1".to_string()),
            tool: "cargo".to_string(),
            status: BuildTestStatus::Success,
            duration_ms: 1200,
            tests_total: 3,
            tests_failed: 0,
            error_kind: None,
        };
        let mut gate = PermissionGate::default();
        gate.set_state(PermissionState {
            git_metadata_enabled: true,
            build_test_events_enabled: true,
            developer_workspace_bound: false,
            ..PermissionState::default()
        });

        assert!(gate.allow(&git_event).is_err());
        assert!(gate.allow(&build_event).is_err());

        gate.set_state(PermissionState {
            git_metadata_enabled: true,
            build_test_events_enabled: true,
            developer_workspace_bound: true,
            ..PermissionState::default()
        });

        assert!(gate.allow(&git_event).is_ok());
        assert!(gate.allow(&build_event).is_ok());
    }

    #[test]
    fn pointer_chase_coordinates_are_allowed_as_local_mouse_input() {
        let gate = PermissionGate::default();

        assert!(gate
            .allow(&IanEvent::MouseChaseCandidate {
                x: 640.0,
                y: 420.0,
                now_ms: 300_000,
            })
            .is_ok());
    }
}
