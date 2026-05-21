use super::reminder_policy::ReminderPolicy;

#[derive(Default)]
pub struct ReminderEngine {
    policy: ReminderPolicy,
}

impl ReminderEngine {
    pub fn is_enabled(&self) -> bool {
        self.policy.enabled
    }
}
