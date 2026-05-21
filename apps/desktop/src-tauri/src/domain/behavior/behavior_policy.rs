pub struct BehaviorPolicy {
    click_phrases: [&'static str; 4],
    cute_phrases: [&'static str; 12],
    run_around_duration_ms: u64,
    attention_cooldown_ms: i64,
}

const DESKTOP_TICK_WINDOW_SECS: i64 = 15;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MovementProfile {
    pub roam_interval_secs: Option<i64>,
    pub roam_offset_x: f64,
    pub roam_offset_y: f64,
}

#[derive(Debug, Clone)]
pub struct ControlledRandom {
    state: u64,
}

impl Default for BehaviorPolicy {
    fn default() -> Self {
        Self {
            click_phrases: ["我在这儿。", "哞？", "喝水水。", "才不是担心你。"],
            cute_phrases: [
                "我在这儿。",
                "再摸摸也可以。",
                "嘿嘿，在呢。",
                "耳朵有点痒。",
                "我靠近一点。",
                "被发现开心。",
                "轻一点嘛。",
                "尾巴要摇起来了。",
                "我精神啦。",
                "抱一下桌角。",
                "这下舒服了。",
                "我会乖一会儿。",
            ],
            run_around_duration_ms: 1800,
            attention_cooldown_ms: 5_000,
        }
    }
}

impl BehaviorPolicy {
    pub fn click_phrase(&self) -> &'static str {
        self.click_phrases[0]
    }

    pub fn run_around_duration_ms(&self) -> u64 {
        self.run_around_duration_ms
    }

    pub fn attention_cooldown_ms(&self) -> i64 {
        self.attention_cooldown_ms
    }

    pub fn affectionate_phrase(&self, interaction_count: u32) -> &'static str {
        let index = interaction_count.saturating_sub(1) as usize % self.cute_phrases.len();
        self.cute_phrases[index]
    }

    pub fn playful_trigger_interval_secs(
        &self,
        energy: &crate::protocol::PlayfulEnergy,
    ) -> Option<i64> {
        match energy {
            crate::protocol::PlayfulEnergy::Off => None,
            crate::protocol::PlayfulEnergy::Low => Some(360),
            crate::protocol::PlayfulEnergy::Normal => Some(240),
            crate::protocol::PlayfulEnergy::High => Some(180),
        }
    }

    pub fn playful_effect_intensity(
        &self,
        energy: &crate::protocol::PlayfulEnergy,
    ) -> &'static str {
        match energy {
            crate::protocol::PlayfulEnergy::Off | crate::protocol::PlayfulEnergy::Low => "low",
            crate::protocol::PlayfulEnergy::Normal => "medium",
            crate::protocol::PlayfulEnergy::High => "high",
        }
    }

    pub fn tick_animation(&self, now_ms: i64) -> (&'static str, bool) {
        let second = now_ms.div_euclid(1000);

        if second >= 90 && second.rem_euclid(90) < DESKTOP_TICK_WINDOW_SECS {
            return ("sleep", true);
        }

        if second >= 45 && second.rem_euclid(45) < DESKTOP_TICK_WINDOW_SECS {
            return ("rest", true);
        }

        ("idle", true)
    }

    pub fn movement_profile(&self, mode: &crate::protocol::BehaviorMode) -> MovementProfile {
        match mode {
            crate::protocol::BehaviorMode::Quiet => MovementProfile {
                roam_interval_secs: None,
                roam_offset_x: 0.0,
                roam_offset_y: 0.0,
            },
            crate::protocol::BehaviorMode::Normal => MovementProfile {
                roam_interval_secs: Some(135),
                roam_offset_x: 64.0,
                roam_offset_y: 28.0,
            },
            crate::protocol::BehaviorMode::Lively => MovementProfile {
                roam_interval_secs: Some(75),
                roam_offset_x: 96.0,
                roam_offset_y: 42.0,
            },
        }
    }

    pub fn should_reduce_disturbance_for_app_category(&self, category: &str) -> bool {
        matches!(category, "meeting" | "focus" | "presentation")
    }

    pub fn should_emit_micro_motion(
        &self,
        now_ms: i64,
        mode: &crate::protocol::BehaviorMode,
    ) -> bool {
        if matches!(mode, crate::protocol::BehaviorMode::Quiet) {
            return false;
        }

        let second = now_ms.div_euclid(1000);
        second >= 15 && second.rem_euclid(15) < 8
    }
}

impl ControlledRandom {
    pub fn seeded(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn next_index(&mut self, len: usize) -> usize {
        if len == 0 {
            return 0;
        }

        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        (self.state % len as u64) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::{BehaviorPolicy, ControlledRandom};
    use crate::protocol::BehaviorMode;

    #[test]
    fn active_app_category_can_reduce_disturbance_without_window_content() {
        let policy = BehaviorPolicy::default();

        assert!(policy.should_reduce_disturbance_for_app_category("meeting"));
        assert!(!policy.should_reduce_disturbance_for_app_category("editor"));
    }

    #[test]
    fn movement_profiles_distinguish_behavior_modes() {
        let policy = BehaviorPolicy::default();

        assert_eq!(
            policy
                .movement_profile(&BehaviorMode::Quiet)
                .roam_interval_secs,
            None
        );
        assert_eq!(
            policy
                .movement_profile(&BehaviorMode::Normal)
                .roam_interval_secs,
            Some(135)
        );
        assert_eq!(
            policy
                .movement_profile(&BehaviorMode::Lively)
                .roam_interval_secs,
            Some(75)
        );
    }

    #[test]
    fn controlled_random_is_repeatable_from_seed() {
        let mut first = ControlledRandom::seeded(42);
        let mut second = ControlledRandom::seeded(42);

        let first_sequence: Vec<usize> = (0..5).map(|_| first.next_index(7)).collect();
        let second_sequence: Vec<usize> = (0..5).map(|_| second.next_index(7)).collect();

        assert_eq!(first_sequence, second_sequence);
        assert!(first_sequence.iter().all(|index| *index < 7));
    }
}
