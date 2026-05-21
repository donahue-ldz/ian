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
}
