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
    workspace_id: Option<String>,
    queued: Vec<BuildTestSummary>,
}

impl BuildTestAdapter {
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn bind_workspace(&mut self, workspace_id: String) {
        self.workspace_id = Some(workspace_id);
    }

    pub fn push_summary(&mut self, summary: BuildTestSummary) {
        self.queued.push(summary);
    }
}

#[cfg(test)]
mod tests {
    use super::{BuildTestAdapter, BuildTestSummary};
    use crate::{
        adapters::PerceptionAdapter,
        protocol::{BuildTestStatus, IanEvent},
    };

    #[test]
    fn build_test_adapter_requires_workspace_binding() {
        let mut adapter = BuildTestAdapter::default();
        adapter.enable();
        adapter.push_summary(BuildTestSummary {
            tool: "cargo".to_string(),
            status: BuildTestStatus::Success,
            duration_ms: 1200,
            tests_total: 3,
            tests_failed: 0,
            error_kind: None,
        });

        assert!(adapter.poll().is_empty());
    }

    #[test]
    fn build_test_adapter_emits_structured_summary_for_bound_workspace() {
        let mut adapter = BuildTestAdapter::default();
        adapter.enable();
        adapter.bind_workspace("workspace-1".to_string());
        adapter.push_summary(BuildTestSummary {
            tool: "cargo".to_string(),
            status: BuildTestStatus::Failure,
            duration_ms: 1200,
            tests_total: 3,
            tests_failed: 1,
            error_kind: Some("test_failed".to_string()),
        });

        let events = adapter.poll();

        assert!(matches!(
            events.first(),
            Some(IanEvent::DeveloperBuildTestSummary {
                workspace_id,
                tool,
                status: BuildTestStatus::Failure,
                tests_failed: 1,
                ..
            }) if workspace_id.as_deref() == Some("workspace-1") && tool == "cargo"
        ));
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
        let Some(workspace_id) = self.workspace_id.clone() else {
            self.queued.clear();
            return Vec::new();
        };

        if !self.enabled {
            self.queued.clear();
            return Vec::new();
        }

        std::mem::take(&mut self.queued)
            .into_iter()
            .map(|summary| IanEvent::DeveloperBuildTestSummary {
                workspace_id: Some(workspace_id.clone()),
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
