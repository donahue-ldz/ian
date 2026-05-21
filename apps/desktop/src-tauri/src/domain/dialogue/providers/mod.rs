pub mod demo;
pub mod openai_compatible;

use crate::{
    domain::{bond::BondStateView, mood::MoodState},
    protocol::CurrentBehavior,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogueSource {
    UserBubble,
}

pub struct DialogueContext {
    pub current_behavior: CurrentBehavior,
    pub current_animation: String,
    pub source: DialogueSource,
    pub mood: MoodState,
    pub bond: BondStateView,
    pub day_phase: String,
    pub interaction_count: u32,
    pub recent_activity_level: String,
}

pub trait DialogueProvider: Send + Sync {
    fn reply(&self, input: &str, context: &DialogueContext) -> String;
}
