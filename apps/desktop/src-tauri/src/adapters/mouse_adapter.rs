use crate::protocol::IanEvent;

use super::{PerceptionAdapter, SensitivityLevel};

#[derive(Default)]
pub struct MouseAdapter {
    queued: Vec<IanEvent>,
}

impl MouseAdapter {
    pub fn push(&mut self, event: IanEvent) {
        self.queued.push(event);
    }
}

impl PerceptionAdapter for MouseAdapter {
    fn id(&self) -> &'static str {
        "mouse"
    }

    fn sensitivity(&self) -> SensitivityLevel {
        SensitivityLevel::Low
    }

    fn poll(&mut self) -> Vec<IanEvent> {
        std::mem::take(&mut self.queued)
    }
}
