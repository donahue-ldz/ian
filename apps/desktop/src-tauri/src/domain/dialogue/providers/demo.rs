#[derive(Default)]
pub struct DemoDialogueProvider;

use crate::domain::{bond::BondStateView, mood::MoodState};

use super::{DialogueContext, DialogueProvider};

impl DialogueProvider for DemoDialogueProvider {
    fn reply(&self, text: &str, context: &DialogueContext) -> String {
        let lowered = text.to_lowercase();

        if context
            .confirmed_memory_tags
            .iter()
            .any(|tag| tag == "pref:quiet")
            && (text.contains("记得") || lowered.contains("remember"))
        {
            return "安静一点，我记得。".to_string();
        }

        if context.day_phase == "night" {
            return "小声陪你。".to_string();
        }

        if lowered.contains("water") || text.contains('水') {
            return "喝水水。".to_string();
        }

        if lowered.contains("sleep") || text.contains('困') {
            return "陪你一会儿就睡。".to_string();
        }

        if matches!(context.bond, BondStateView::Familiar) {
            return "我在，老朋友。".to_string();
        }

        if matches!(context.mood, MoodState::Sleepy) {
            return "我有点困，但还在。".to_string();
        }

        "我在这儿。".to_string()
    }
}
