use crate::protocol::{BuildTestStatus, IanEvent};

use super::{PerceptionAdapter, SensitivityLevel};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildTestSummary {
    pub tool: String,
    pub status: BuildTestStatus,
    pub duration_ms: u64,
    pub tests_total: u32,
    pub tests_failed: u32,
    pub error_kind: Option<String>,
}

#[derive(Default)]
pub struct BuildTestAdapter {
    enabled: bool,
    queued: Vec<BuildTestSummary>,
}

impl BuildTestAdapter {
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn push_summary(&mut self, summary: BuildTestSummary) {
        self.queued.push(summary);
    }
}

impl PerceptionAdapter for BuildTestAdapter {
    fn id(&self) -> &'static str {
        "developer.build_test"
    }

    fn sensitivity(&self) -> SensitivityLevel {
        SensitivityLevel::Medium
    }

    fn poll(&mut self) -> Vec<IanEvent> {
        if !self.enabled {
            self.queued.clear();
            return Vec::new();
        }

        std::mem::take(&mut self.queued)
            .into_iter()
            .map(|summary| IanEvent::DeveloperBuildTestSummary {
                tool: summary.tool,
                status: summary.status,
                duration_ms: summary.duration_ms,
                tests_total: summary.tests_total,
                tests_failed: summary.tests_failed,
                error_kind: summary.error_kind,
            })
            .collect()
    }
}
