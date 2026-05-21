pub struct BehaviorPolicy {
    click_phrases: [&'static str; 4],
    run_around_duration_ms: u64,
}

impl Default for BehaviorPolicy {
    fn default() -> Self {
        Self {
            click_phrases: ["我在这儿。", "哞？", "喝水水。", "才不是担心你。"],
            run_around_duration_ms: 1800,
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

    pub fn affectionate_phrase(&self, interaction_count: u32) -> &'static str {
        if interaction_count >= 2 {
            "再摸摸也可以。"
        } else {
            self.click_phrase()
        }
    }

    pub fn tick_animation(&self, now_ms: i64) -> (&'static str, bool) {
        let second = now_ms.div_euclid(1000);

        if second > 0 && second % 90 == 0 {
            return ("sleep", true);
        }

        if second > 0 && second % 45 == 0 {
            return ("walk", false);
        }

        ("idle", true)
    }

    pub fn should_reduce_disturbance_for_app_category(&self, category: &str) -> bool {
        matches!(category, "meeting" | "focus" | "presentation")
    }
}

#[cfg(test)]
mod tests {
    use super::BehaviorPolicy;

    #[test]
    fn active_app_category_can_reduce_disturbance_without_window_content() {
        let policy = BehaviorPolicy::default();

        assert!(policy.should_reduce_disturbance_for_app_category("meeting"));
        assert!(!policy.should_reduce_disturbance_for_app_category("editor"));
    }
}
