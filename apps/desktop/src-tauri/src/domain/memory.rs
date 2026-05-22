#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProposedMemoryCandidate {
    pub tags: String,
    pub created_at_ms: i64,
}

#[derive(Debug, Clone)]
pub struct MemoryCandidatePolicy {
    min_interval_ms: i64,
}

impl Default for MemoryCandidatePolicy {
    fn default() -> Self {
        Self {
            min_interval_ms: 60 * 60 * 1000,
        }
    }
}

impl MemoryCandidatePolicy {
    pub fn candidate_from_user_text(
        &self,
        text: &str,
        now_ms: i64,
        existing_tags: &[&str],
    ) -> Option<ProposedMemoryCandidate> {
        if now_ms > 0 && now_ms < self.min_interval_ms {
            return None;
        }
        if looks_sensitive(text) {
            return None;
        }

        let lowered = text.to_lowercase();
        let mut tags = Vec::new();

        if lowered.contains("quiet") || text.contains("安静") {
            tags.push("pref:quiet");
        }
        if lowered.contains("water") || text.contains('水') {
            tags.push("topic:water");
        }
        if lowered.contains("stretch") || text.contains("伸") {
            tags.push("routine:stretch");
        }
        if lowered.contains("morning") || text.contains("早") {
            tags.push("moment:morning");
        }

        tags.sort_unstable();
        tags.dedup();
        if tags.is_empty() || tags.iter().any(|tag| existing_tags.contains(tag)) {
            return None;
        }

        Some(ProposedMemoryCandidate {
            tags: tags.join(","),
            created_at_ms: now_ms,
        })
    }
}

fn looks_sensitive(text: &str) -> bool {
    let lowered = text.to_lowercase();
    lowered.contains("/users/")
        || lowered.contains("fn ")
        || lowered.contains("private")
        || lowered.contains("secret")
        || lowered.contains("私聊")
        || text.contains('<')
        || text.contains('>')
        || text.contains('{')
        || text.contains('}')
        || text.contains('\\')
}

#[cfg(test)]
mod tests {
    use super::MemoryCandidatePolicy;

    #[test]
    fn memory_policy_only_proposes_allowlisted_low_sensitive_tags() {
        let policy = MemoryCandidatePolicy::default();

        let candidate = policy
            .candidate_from_user_text("以后安静一点陪我，也提醒我喝水", 3_700_000, &[])
            .expect("candidate");

        assert_eq!(candidate.tags, "pref:quiet,topic:water");
        assert_eq!(candidate.created_at_ms, 3_700_000);

        for rejected in [
            "记住 /Users/alice/private/project",
            "记住 fn main() { panic!() }",
            "<script>alert(1)</script>",
            "我的私聊内容是 secret",
        ] {
            assert!(policy
                .candidate_from_user_text(rejected, 2_000, &[])
                .is_none());
        }
    }

    #[test]
    fn memory_policy_deduplicates_and_rate_limits_candidates() {
        let policy = MemoryCandidatePolicy::default();

        assert!(policy
            .candidate_from_user_text("提醒我喝水", 1_000, &["topic:water"])
            .is_none());
        assert!(policy
            .candidate_from_user_text("提醒我喝水", 1_000 + 30_000, &[])
            .is_none());
        assert!(policy
            .candidate_from_user_text("提醒我喝水", 1_000 + 3_700_000, &[])
            .is_some());
    }
}
