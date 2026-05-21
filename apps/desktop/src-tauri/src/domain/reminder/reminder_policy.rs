#[derive(Debug, Clone, Copy)]
pub struct ReminderPolicy {
    pub enabled: bool,
    pub cooldown_ms: i64,
}

impl Default for ReminderPolicy {
    fn default() -> Self {
        Self {
            enabled: true,
            cooldown_ms: 90 * 60 * 1000,
        }
    }
}

impl ReminderPolicy {
    pub fn should_reduce_for_app_category(&self, category: Option<&str>) -> bool {
        matches!(category, Some("meeting" | "presentation" | "focus"))
    }
}

#[cfg(test)]
mod tests {
    use super::ReminderPolicy;

    #[test]
    fn default_policy_allows_gentle_local_reminders() {
        let policy = ReminderPolicy::default();

        assert!(policy.enabled);
        assert_eq!(policy.cooldown_ms, 5_400_000);
    }

    #[test]
    fn busy_app_category_reduces_reminders_without_window_content() {
        let policy = ReminderPolicy::default();

        assert!(policy.should_reduce_for_app_category(Some("meeting")));
        assert!(!policy.should_reduce_for_app_category(Some("editor")));
    }
}
