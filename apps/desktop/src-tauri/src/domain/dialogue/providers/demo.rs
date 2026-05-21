#[derive(Default)]
pub struct DemoDialogueProvider;

impl DemoDialogueProvider {
    pub fn reply(&self, text: &str) -> String {
        let lowered = text.to_lowercase();

        if lowered.contains("water") || text.contains('水') {
            return "喝水水。".to_string();
        }

        if lowered.contains("sleep") || text.contains('困') {
            return "陪你一会儿就睡。".to_string();
        }

        "我在这儿。".to_string()
    }
}
