pub struct OpenAiCompatibleProviderConfig {
    pub enabled: bool,
    pub base_url: String,
    pub model: String,
    pub api_key_ref: Option<String>,
}

impl Default for OpenAiCompatibleProviderConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            base_url: "https://api.openai.com/v1".to_string(),
            model: "gpt-4o-mini".to_string(),
            api_key_ref: None,
        }
    }
}

impl OpenAiCompatibleProviderConfig {
    pub fn can_make_network_request(&self) -> bool {
        self.enabled && self.api_key_ref.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::OpenAiCompatibleProviderConfig;

    #[test]
    fn byom_config_defaults_to_no_network() {
        let config = OpenAiCompatibleProviderConfig::default();

        assert!(!config.enabled);
        assert!(!config.can_make_network_request());
    }
}
