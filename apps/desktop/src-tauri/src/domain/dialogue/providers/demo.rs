#[derive(Default)]
pub struct DemoDialogueProvider;

use crate::domain::{bond::BondStateView, mood::MoodState};

use super::{DialogueContext, DialogueProvider};

impl DialogueProvider for DemoDialogueProvider {
    fn reply(&self, text: &str, context: &DialogueContext) -> String {
        let lowered = text.to_lowercase();

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
