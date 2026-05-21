use crate::protocol::IanAction;

use super::{dialogue_policy::DialoguePolicy, providers::demo::DemoDialogueProvider};

pub struct DialogueEngine {
    demo: DemoDialogueProvider,
    policy: DialoguePolicy,
}

impl Default for DialogueEngine {
    fn default() -> Self {
        Self {
            demo: DemoDialogueProvider::default(),
            policy: DialoguePolicy::default(),
        }
    }
}

impl DialogueEngine {
    pub fn reply_to(&self, text: String) -> Vec<IanAction> {
        let reply = self.policy.trim_for_bubble(self.demo.reply(&text));

        vec![
            IanAction::BubbleOpen,
            IanAction::SpeechShow {
                text: reply,
                mood: Some("calm".to_string()),
                duration_ms: Some(2600),
            },
            IanAction::AnimationPlay {
                name: "happy".to_string(),
                looped: false,
            },
        ]
    }
}
