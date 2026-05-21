use crate::protocol::IanEvent;

use super::{PerceptionAdapter, SensitivityLevel};

#[derive(Debug, Clone, PartialEq)]
pub struct ActiveAppPresenceSummary {
    pub category: String,
    pub confidence: f32,
    pub app_id: Option<String>,
}

#[derive(Default)]
pub struct ActiveAppPresenceAdapter {
    enabled: bool,
    queued: Vec<ActiveAppPresenceSummary>,
}

impl ActiveAppPresenceAdapter {
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn push_summary(&mut self, summary: ActiveAppPresenceSummary) {
        self.queued.push(summary);
    }
}

impl PerceptionAdapter for ActiveAppPresenceAdapter {
    fn id(&self) -> &'static str {
        "active_app.presence"
    }

    fn sensitivity(&self) -> SensitivityLevel {
        SensitivityLevel::High
    }

    fn poll(&mut self) -> Vec<IanEvent> {
        if !self.enabled {
            self.queued.clear();
            return Vec::new();
        }

        std::mem::take(&mut self.queued)
            .into_iter()
            .map(|summary| IanEvent::ActiveAppPresence {
                category: summary.category,
                confidence: summary.confidence,
                app_id: summary.app_id,
            })
            .collect()
    }
}
