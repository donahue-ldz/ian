#[derive(Debug, Clone, Copy)]
pub struct ReminderPolicy {
    pub enabled: bool,
}

impl Default for ReminderPolicy {
    fn default() -> Self {
        Self { enabled: false }
    }
}
