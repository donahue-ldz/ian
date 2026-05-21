use crate::protocol::IanEvent;

use super::{PerceptionAdapter, SensitivityLevel};

#[derive(Default)]
pub struct DialogueAdapter {
    queued: Vec<IanEvent>,
}

impl DialogueAdapter {
    pub fn push_user_message(&mut self, text: String) {
        self.queued.push(IanEvent::DialogueUserMessage { text });
    }
}

impl PerceptionAdapter for DialogueAdapter {
    fn id(&self) -> &'static str {
        "dialogue"
    }

    fn sensitivity(&self) -> SensitivityLevel {
        SensitivityLevel::Low
    }

    fn poll(&mut self) -> Vec<IanEvent> {
        std::mem::take(&mut self.queued)
    }
}
