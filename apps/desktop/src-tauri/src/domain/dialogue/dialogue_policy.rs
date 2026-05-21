pub struct DialoguePolicy {
    max_chars: usize,
}

impl Default for DialoguePolicy {
    fn default() -> Self {
        Self { max_chars: 18 }
    }
}

impl DialoguePolicy {
    pub fn with_max_chars(max_chars: usize) -> Self {
        Self { max_chars }
    }

    pub fn apply(&self, text: String) -> String {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return "我在这儿。".to_string();
        }

        let identity_safe = trimmed
            .replace("AI assistant", "Ian")
            .replace("人工智能助手", "Ian")
            .replace("ChatGPT", "Ian");

        identity_safe.chars().take(self.max_chars).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::DialoguePolicy;

    #[test]
    fn policy_falls_back_for_empty_output() {
        assert_eq!(
            DialoguePolicy::default().apply("   ".to_string()),
            "我在这儿。"
        );
    }

    #[test]
    fn policy_trims_and_removes_assistant_identity() {
        let policy = DialoguePolicy::with_max_chars(12);

        let reply = policy.apply("我是 AI assistant，会陪你。".to_string());

        assert!(!reply.contains("assistant"));
        assert!(reply.chars().count() <= 12);
    }
}
