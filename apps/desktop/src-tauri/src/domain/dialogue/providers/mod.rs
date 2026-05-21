pub mod demo;
pub mod openai_compatible;

use crate::protocol::CurrentBehavior;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogueSource {
    UserBubble,
}

pub struct DialogueContext {
    pub current_behavior: CurrentBehavior,
    pub current_animation: String,
    pub source: DialogueSource,
}

pub trait DialogueProvider: Send + Sync {
    fn reply(&self, input: &str, context: &DialogueContext) -> String;
}
