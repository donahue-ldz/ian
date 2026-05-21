pub struct OpenAiCompatibleProviderConfig {
    pub enabled: bool,
    pub base_url: String,
    pub model: String,
}

impl Default for OpenAiCompatibleProviderConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            base_url: "https://api.openai.com/v1".to_string(),
            model: "gpt-4o-mini".to_string(),
        }
    }
}
