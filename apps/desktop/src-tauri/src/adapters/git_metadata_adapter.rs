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
    queued: Vec<GitMetadataSnapshot>,
}

impl GitMetadataAdapter {
    pub fn new_disabled() -> Self {
        Self::default()
    }

    pub fn enable(&mut self) {
        self.enabled = true;
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
        if !self.enabled {
            self.queued.clear();
            return Vec::new();
        }

        std::mem::take(&mut self.queued)
            .into_iter()
            .map(|snapshot| IanEvent::DeveloperGitStatusChanged {
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
        adapter.push_snapshot(GitMetadataSnapshot {
            branch: "main".to_string(),
            dirty: true,
            short_commit: "abc1234".to_string(),
        });

        let events = adapter.poll();

        assert!(matches!(
            events.first(),
            Some(IanEvent::DeveloperGitStatusChanged {
                branch,
                dirty: true,
                short_commit,
            }) if branch == "main" && short_commit == "abc1234"
        ));
    }
}
