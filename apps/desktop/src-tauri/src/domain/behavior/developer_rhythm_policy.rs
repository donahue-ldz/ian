use crate::protocol::{BuildTestStatus, IanAction, IanEvent};

#[derive(Default)]
pub struct DeveloperRhythmPolicy {
    last_response_ms: Option<i64>,
    consecutive_failures: u32,
}

impl DeveloperRhythmPolicy {
    const COOLDOWN_MS: i64 = 30_000;

    pub fn actions_for_event(&mut self, event: &IanEvent, now_ms: i64) -> Vec<IanAction> {
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
    use crate::protocol::{BuildTestStatus, IanAction, IanEvent};

    #[test]
    fn policy_uses_cooldown_for_developer_events() {
        let mut policy = DeveloperRhythmPolicy::default();
        let event = IanEvent::DeveloperBuildTestSummary {
            tool: "cargo".to_string(),
            status: BuildTestStatus::Success,
            duration_ms: 1000,
            tests_total: 3,
            tests_failed: 0,
            error_kind: None,
        };

        assert!(!policy.actions_for_event(&event, 1_000).is_empty());
        assert!(policy.actions_for_event(&event, 2_000).is_empty());
        assert!(!policy.actions_for_event(&event, 31_000).is_empty());
    }

    #[test]
    fn consecutive_failures_get_a_gentler_reaction() {
        let mut policy = DeveloperRhythmPolicy::default();
        let event = IanEvent::DeveloperBuildTestSummary {
            tool: "cargo".to_string(),
            status: BuildTestStatus::Failure,
            duration_ms: 1000,
            tests_total: 3,
            tests_failed: 1,
            error_kind: Some("test_failed".to_string()),
        };

        let first = policy.actions_for_event(&event, 1_000);
        let second = policy.actions_for_event(&event, 31_000);

        assert!(first.iter().any(|action| matches!(
            action,
            IanAction::SpeechShow { text, .. } if text == "卡住了吗？"
        )));
        assert!(second.iter().any(|action| matches!(
            action,
            IanAction::SpeechShow { text, .. } if text == "别急，慢慢来。"
        )));
    }
}
