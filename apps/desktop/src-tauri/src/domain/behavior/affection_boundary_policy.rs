use crate::protocol::IanState;

#[derive(Debug, Default)]
pub struct AffectionBoundaryPolicy;

impl AffectionBoundaryPolicy {
    pub fn validate_phrase(&self, phrase: &str) -> Result<(), String> {
        const FORBIDDEN: &[&str] = &["不要离开我", "你不理我", "我会难过", "必须陪我"];
        if FORBIDDEN.iter().any(|term| phrase.contains(term)) {
            Err("affection phrase has coercive wording".to_string())
        } else {
            Ok(())
        }
    }

    pub fn can_emit_proactive_affection(&self, enabled: bool, state: &IanState) -> bool {
        enabled && state.reminders_enabled && !state.do_not_disturb && !state.is_bubble_input_active
    }
}

#[cfg(test)]
mod tests {
    use super::AffectionBoundaryPolicy;
    use crate::protocol::IanState;

    #[test]
    fn affection_boundary_rejects_coercive_copy_and_respects_user_off_switches() {
        let policy = AffectionBoundaryPolicy::default();
        let mut state = IanState::default();

        assert!(policy.validate_phrase("我在这里。").is_ok());
        assert!(policy.validate_phrase("你不理我我会难过").is_err());
        assert!(policy.can_emit_proactive_affection(true, &state));

        state.do_not_disturb = true;
        assert!(!policy.can_emit_proactive_affection(true, &state));
        state.do_not_disturb = false;
        assert!(!policy.can_emit_proactive_affection(false, &state));
    }
}
