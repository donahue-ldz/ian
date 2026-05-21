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

    pub fn trim_for_bubble(&self, text: String) -> String {
        text.chars().take(self.max_chars).collect()
    }
}
