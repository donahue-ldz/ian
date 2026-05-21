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

#[cfg(test)]
mod tests {
    use super::ReminderPolicy;

    #[test]
    fn default_policy_allows_gentle_local_reminders() {
        let policy = ReminderPolicy::default();

        assert!(policy.enabled);
        assert_eq!(policy.cooldown_ms, 5_400_000);
    }
}
