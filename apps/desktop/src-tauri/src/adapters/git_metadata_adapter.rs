use crate::protocol::IanEvent;

use super::{PerceptionAdapter, SensitivityLevel};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitMetadataSnapshot {
    pub branch: String,
    pub dirty: bool,
    pub short_commit: String,
}

#[derive(Default)]
pub struct GitMetadataAdapter {
    enabled: bool,
    workspace_id: Option<String>,
    queued: Vec<GitMetadataSnapshot>,
}

impl GitMetadataAdapter {
    pub fn new_disabled() -> Self {
        Self::default()
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn bind_workspace(&mut self, workspace_id: String) {
        self.workspace_id = Some(workspace_id);
    }

    pub fn push_snapshot(&mut self, snapshot: GitMetadataSnapshot) {
        self.queued.push(snapshot);
    }
}

impl PerceptionAdapter for GitMetadataAdapter {
    fn id(&self) -> &'static str {
        "developer.git_metadata"
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
            .map(|snapshot| IanEvent::DeveloperGitStatusChanged {
                workspace_id: Some(workspace_id.clone()),
                branch: snapshot.branch,
                dirty: snapshot.dirty,
                short_commit: snapshot.short_commit,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{GitMetadataAdapter, GitMetadataSnapshot};
    use crate::{adapters::PerceptionAdapter, protocol::IanEvent};

    #[test]
    fn git_metadata_adapter_is_disabled_by_default() {
        let mut adapter = GitMetadataAdapter::new_disabled();
        adapter.push_snapshot(GitMetadataSnapshot {
            branch: "main".to_string(),
            dirty: true,
            short_commit: "abc1234".to_string(),
        });

        assert!(adapter.poll().is_empty());
    }

    #[test]
    fn git_metadata_adapter_emits_low_sensitive_metadata_only_when_enabled() {
        let mut adapter = GitMetadataAdapter::new_disabled();
        adapter.enable();
        adapter.bind_workspace("workspace-1".to_string());
        adapter.push_snapshot(GitMetadataSnapshot {
            branch: "main".to_string(),
            dirty: true,
            short_commit: "abc1234".to_string(),
        });

        let events = adapter.poll();

        assert!(matches!(
            events.first(),
            Some(IanEvent::DeveloperGitStatusChanged {
                workspace_id,
                branch,
                dirty: true,
                short_commit,
            }) if workspace_id.as_deref() == Some("workspace-1") && branch == "main" && short_commit == "abc1234"
        ));
    }

    #[test]
    fn git_metadata_adapter_requires_workspace_binding() {
        let mut adapter = GitMetadataAdapter::new_disabled();
        adapter.enable();
        adapter.push_snapshot(GitMetadataSnapshot {
            branch: "main".to_string(),
            dirty: false,
            short_commit: "abc1234".to_string(),
        });

        assert!(adapter.poll().is_empty());
    }
}
