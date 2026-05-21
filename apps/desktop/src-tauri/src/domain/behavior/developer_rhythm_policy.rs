use crate::protocol::{BuildTestStatus, IanAction, IanEvent, IanState};

#[derive(Default)]
pub struct DeveloperRhythmPolicy {
    last_response_ms: Option<i64>,
    consecutive_failures: u32,
}

impl DeveloperRhythmPolicy {
    const COOLDOWN_MS: i64 = 30_000;

    pub fn actions_for_event(
        &mut self,
        event: &IanEvent,
        now_ms: i64,
        state: &IanState,
    ) -> Vec<IanAction> {
        if self.should_suppress(now_ms, state) {
            return Vec::new();
        }

        if !self.can_respond(now_ms) {
            return Vec::new();
        }

        let actions = match event {
            IanEvent::DeveloperBuildTestSummary {
                status: BuildTestStatus::Success,
                ..
            } => {
                self.consecutive_failures = 0;
                self.short_reaction("过啦。", "happy")
            }
            IanEvent::DeveloperBuildTestSummary {
                status: BuildTestStatus::Failure,
                ..
            } => {
                self.consecutive_failures += 1;
                if self.consecutive_failures >= 2 {
                    self.short_reaction("别急，慢慢来。", "sleep")
                } else {
                    self.short_reaction("卡住了吗？", "happy")
                }
            }
            IanEvent::DeveloperGitStatusChanged { dirty: true, .. } => {
                self.short_reaction("我看着就好。", "idle")
            }
            IanEvent::DeveloperGitStatusChanged { dirty: false, .. } => {
                self.short_reaction("收好啦。", "happy")
            }
            _ => Vec::new(),
        };

        if !actions.is_empty() {
            self.last_response_ms = Some(now_ms);
        }

        actions
    }

    fn can_respond(&self, now_ms: i64) -> bool {
        self.last_response_ms
            .map(|last| now_ms - last >= Self::COOLDOWN_MS)
            .unwrap_or(true)
    }

    fn should_suppress(&self, now_ms: i64, state: &IanState) -> bool {
        state.developer_snooze.is_active_at(now_ms)
            || matches!(
                state.active_app_category.as_deref(),
                Some("meeting" | "presentation" | "focus")
            )
            || state.quiet_hours.is_active_at_minute(
                crate::domain::behavior::day_phase_policy::minute_of_day_from_epoch_ms(now_ms),
            )
    }

    fn short_reaction(&self, text: &str, animation: &str) -> Vec<IanAction> {
        vec![
            IanAction::BubbleOpen,
            IanAction::SpeechShow {
                text: text.to_string(),
                mood: Some("calm".to_string()),
                duration_ms: Some(2600),
            },
            IanAction::AnimationPlay {
                name: animation.to_string(),
                looped: false,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::DeveloperRhythmPolicy;
    use crate::protocol::{BuildTestStatus, DeveloperSnooze, IanAction, IanEvent, IanState};

    #[test]
    fn policy_uses_cooldown_for_developer_events() {
        let mut policy = DeveloperRhythmPolicy::default();
        let event = IanEvent::DeveloperBuildTestSummary {
            workspace_id: Some("workspace-1".to_string()),
            tool: "cargo".to_string(),
            status: BuildTestStatus::Success,
            duration_ms: 1000,
            tests_total: 3,
            tests_failed: 0,
            error_kind: None,
        };

        assert!(!policy
            .actions_for_event(&event, 1_000, &IanState::default())
            .is_empty());
        assert!(policy
            .actions_for_event(&event, 2_000, &IanState::default())
            .is_empty());
        assert!(!policy
            .actions_for_event(&event, 31_000, &IanState::default())
            .is_empty());
    }

    #[test]
    fn consecutive_failures_get_a_gentler_reaction() {
        let mut policy = DeveloperRhythmPolicy::default();
        let event = IanEvent::DeveloperBuildTestSummary {
            workspace_id: Some("workspace-1".to_string()),
            tool: "cargo".to_string(),
            status: BuildTestStatus::Failure,
            duration_ms: 1000,
            tests_total: 3,
            tests_failed: 1,
            error_kind: Some("test_failed".to_string()),
        };

        let first = policy.actions_for_event(&event, 1_000, &IanState::default());
        let second = policy.actions_for_event(&event, 31_000, &IanState::default());

        assert!(first.iter().any(|action| matches!(
            action,
            IanAction::SpeechShow { text, .. } if text == "卡住了吗？"
        )));
        assert!(second.iter().any(|action| matches!(
            action,
            IanAction::SpeechShow { text, .. } if text == "别急，慢慢来。"
        )));
    }

    #[test]
    fn snooze_and_busy_context_suppress_developer_reactions_without_affecting_clicks() {
        let event = IanEvent::DeveloperBuildTestSummary {
            workspace_id: Some("workspace-1".to_string()),
            tool: "cargo".to_string(),
            status: BuildTestStatus::Success,
            duration_ms: 1000,
            tests_total: 3,
            tests_failed: 0,
            error_kind: None,
        };
        let mut state = IanState::default();
        state.developer_snooze = DeveloperSnooze {
            enabled: true,
            until_ms: Some(60_000),
            reason: Some("manual".to_string()),
        };
        let mut policy = DeveloperRhythmPolicy::default();

        assert!(policy.actions_for_event(&event, 1_000, &state).is_empty());

        state.developer_snooze.enabled = false;
        state.active_app_category = Some("meeting".to_string());
        assert!(policy.actions_for_event(&event, 61_000, &state).is_empty());
    }
}
