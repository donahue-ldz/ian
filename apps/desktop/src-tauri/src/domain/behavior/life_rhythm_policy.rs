use crate::protocol::{BehaviorMode, IanState};

#[derive(Debug, Default)]
pub struct DailyGreetingPolicy {
    last_greeting_day: Option<i64>,
}

impl DailyGreetingPolicy {
    pub fn greeting_for_day(&mut self, day_index: i64, state: &IanState) -> Option<&'static str> {
        if matches!(state.behavior_mode, BehaviorMode::Quiet)
            || state.do_not_disturb
            || self.last_greeting_day == Some(day_index)
        {
            return None;
        }
        self.last_greeting_day = Some(day_index);
        if day_index % 2 == 0 {
            Some("今天也在。")
        } else {
            Some("早呀。")
        }
    }
}

#[derive(Debug)]
pub struct AbsenceReturnPolicy {
    threshold_ms: i64,
    cooldown_ms: i64,
    last_reaction_ms: Option<i64>,
}

impl Default for AbsenceReturnPolicy {
    fn default() -> Self {
        Self {
            threshold_ms: 6 * 60 * 60 * 1000,
            cooldown_ms: 12 * 60 * 60 * 1000,
            last_reaction_ms: None,
        }
    }
}

impl AbsenceReturnPolicy {
    pub fn reaction(&mut self, last_interaction_ms: i64, now_ms: i64) -> Option<&'static str> {
        if now_ms.saturating_sub(last_interaction_ms) < self.threshold_ms {
            return None;
        }
        if self
            .last_reaction_ms
            .map(|last| now_ms.saturating_sub(last) < self.cooldown_ms)
            .unwrap_or(false)
        {
            return None;
        }
        self.last_reaction_ms = Some(now_ms);
        Some("又见到你了。")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersonalityTraitConfig {
    pub social_energy: String,
    pub affection_style: String,
}

impl Default for PersonalityTraitConfig {
    fn default() -> Self {
        Self {
            social_energy: "normal".to_string(),
            affection_style: "gentle".to_string(),
        }
    }
}

impl PersonalityTraitConfig {
    pub fn behavior_frequency_multiplier(&self) -> f32 {
        match self.social_energy.as_str() {
            "quiet" => 0.6,
            "lively" => 1.35,
            _ => 1.0,
        }
    }

    pub fn identity_guardrail(&self) -> &'static str {
        "Ian 是本地优先的桌面数字生命，不是 AI assistant。"
    }
}

#[cfg(test)]
mod tests {
    use super::{AbsenceReturnPolicy, DailyGreetingPolicy, PersonalityTraitConfig};
    use crate::protocol::{BehaviorMode, IanState};

    #[test]
    fn daily_greeting_is_local_once_per_day_and_respects_quiet_mode() {
        let mut policy = DailyGreetingPolicy::default();
        let mut state = IanState::default();

        assert_eq!(policy.greeting_for_day(12, &state), Some("今天也在。"));
        assert_eq!(policy.greeting_for_day(12, &state), None);

        state.behavior_mode = BehaviorMode::Quiet;
        assert_eq!(policy.greeting_for_day(13, &state), None);
        state.behavior_mode = BehaviorMode::Normal;
        state.do_not_disturb = true;
        assert_eq!(policy.greeting_for_day(14, &state), None);
    }

    #[test]
    fn absence_return_reacts_once_without_blame() {
        let mut policy = AbsenceReturnPolicy::default();

        let phrase = policy.reaction(1_000, 7 * 60 * 60 * 1000).expect("welcome");
        assert!(!phrase.contains("怎么才"));
        assert!(!phrase.contains("你不理"));
        assert!(policy.reaction(1_000, 7 * 60 * 60 * 1000 + 1_000).is_none());
    }

    #[test]
    fn personality_trait_changes_frequency_without_changing_identity() {
        let quiet = PersonalityTraitConfig {
            social_energy: "quiet".to_string(),
            ..PersonalityTraitConfig::default()
        };
        let lively = PersonalityTraitConfig {
            social_energy: "lively".to_string(),
            ..PersonalityTraitConfig::default()
        };

        assert!(quiet.behavior_frequency_multiplier() < 1.0);
        assert!(lively.behavior_frequency_multiplier() > 1.0);
        assert!(quiet.identity_guardrail().contains("不是 AI assistant"));
    }
}
