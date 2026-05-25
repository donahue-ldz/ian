use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoveltyCandidate {
    pub kind: &'static str,
    pub variant: &'static str,
    pub phrase: Option<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoveltyRecord {
    pub kind: &'static str,
    pub variant: &'static str,
    pub phrase: Option<&'static str>,
}

impl NoveltyCandidate {
    pub fn record(self) -> NoveltyRecord {
        NoveltyRecord {
            kind: self.kind,
            variant: self.variant,
            phrase: self.phrase,
        }
    }
}

#[derive(Debug)]
pub struct NoveltyPolicy {
    recent: VecDeque<NoveltyRecord>,
    capacity: usize,
}

impl Default for NoveltyPolicy {
    fn default() -> Self {
        Self {
            recent: VecDeque::new(),
            capacity: 6,
        }
    }
}

impl NoveltyPolicy {
    pub fn choose_non_recent<'a>(
        &self,
        candidates: &'a [NoveltyCandidate],
    ) -> Option<&'a NoveltyCandidate> {
        candidates
            .iter()
            .find(|candidate| !self.was_recent(candidate))
            .or_else(|| candidates.first())
    }

    pub fn record(&mut self, record: NoveltyRecord) -> bool {
        if !record.is_low_sensitive() {
            return false;
        }

        if self.recent.len() >= self.capacity {
            self.recent.pop_front();
        }
        self.recent.push_back(record);
        true
    }

    pub fn recent_records_are_low_sensitive_ids(&self) -> bool {
        self.recent.iter().all(NoveltyRecord::is_low_sensitive)
    }

    fn was_recent(&self, candidate: &NoveltyCandidate) -> bool {
        self.recent.iter().any(|record| {
            (record.kind == candidate.kind && record.variant == candidate.variant)
                || record
                    .phrase
                    .zip(candidate.phrase)
                    .map(|(left, right)| left == right)
                    .unwrap_or(false)
        })
    }
}

impl NoveltyRecord {
    fn is_low_sensitive(&self) -> bool {
        is_low_sensitive_id(self.kind)
            && is_low_sensitive_id(self.variant)
            && self.phrase.map(is_low_sensitive_id).unwrap_or(true)
    }
}

fn is_low_sensitive_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_' || byte == b'-'
        })
}

#[cfg(test)]
mod tests {
    use super::{NoveltyCandidate, NoveltyPolicy, NoveltyRecord};

    #[test]
    fn novelty_policy_keeps_only_low_sensitive_enum_ids() {
        let mut policy = NoveltyPolicy::default();

        assert!(policy.record(NoveltyRecord {
            kind: "idle_private_life",
            variant: "idle_peek_around",
            phrase: Some("phrase_peek_01"),
        }));

        assert!(!policy.record(NoveltyRecord {
            kind: "用户今天在写商业计划",
            variant: "idle peek around",
            phrase: Some("原始文本不应该进入 novelty"),
        }));
        assert!(policy.recent_records_are_low_sensitive_ids());
    }

    #[test]
    fn novelty_policy_avoids_recent_candidates_and_falls_back_when_exhausted() {
        let mut policy = NoveltyPolicy::default();
        let candidates = [
            NoveltyCandidate {
                kind: "idle_private_life",
                variant: "idle_peek_around",
                phrase: Some("phrase_peek_01"),
            },
            NoveltyCandidate {
                kind: "idle_private_life",
                variant: "idle_tiny_patrol",
                phrase: Some("phrase_patrol_01"),
            },
        ];

        policy.record(candidates[0].record());
        let fresh = policy
            .choose_non_recent(&candidates)
            .expect("candidate should exist");
        assert_eq!(fresh.variant, "idle_tiny_patrol");

        policy.record(candidates[1].record());
        let fallback = policy
            .choose_non_recent(&candidates)
            .expect("fallback should preserve behavior");
        assert_eq!(fallback.variant, "idle_peek_around");
    }
}
