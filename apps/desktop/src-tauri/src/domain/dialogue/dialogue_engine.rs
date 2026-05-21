use crate::{
    domain::{bond::BondStateView, mood::MoodState},
    protocol::{IanAction, IanState},
};

use super::{
    dialogue_policy::DialoguePolicy,
    providers::{demo::DemoDialogueProvider, DialogueContext, DialogueProvider, DialogueSource},
};

pub struct DialogueEngine {
    provider: Box<dyn DialogueProvider>,
    policy: DialoguePolicy,
}

impl Default for DialogueEngine {
    fn default() -> Self {
        Self {
            provider: Box::<DemoDialogueProvider>::default(),
            policy: DialoguePolicy::default(),
        }
    }
}

impl DialogueEngine {
    pub fn new(provider: Box<dyn DialogueProvider>, policy: DialoguePolicy) -> Self {
        Self { provider, policy }
    }

    pub fn reply_to(
        &self,
        text: String,
        state: &IanState,
        source: DialogueSource,
        mood: MoodState,
        bond: BondStateView,
    ) -> Vec<IanAction> {
        let context = DialogueContext {
            current_behavior: state.current_behavior.clone(),
            current_animation: state.current_animation.clone(),
            source,
            mood,
            bond,
        };
        let reply = self
            .policy
            .trim_for_bubble(self.provider.reply(&text, &context));

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

#[cfg(test)]
mod tests {
    use crate::{
        domain::{
            bond::BondStateView,
            dialogue::providers::{DialogueContext, DialogueProvider, DialogueSource},
            mood::MoodState,
        },
        protocol::{CurrentBehavior, IanAction, IanState},
    };

    use super::{DialogueEngine, DialoguePolicy};

    struct LongReplyProvider;

    impl DialogueProvider for LongReplyProvider {
        fn reply(&self, input: &str, context: &DialogueContext) -> String {
            assert_eq!(input, "你在干嘛");
            assert_eq!(context.source, DialogueSource::UserBubble);
            assert_eq!(context.current_animation, "idle");
            assert!(matches!(context.current_behavior, CurrentBehavior::Idle));
            assert!(matches!(context.mood, MoodState::Happy));
            assert!(matches!(context.bond, BondStateView::GettingCloser));
            "我正在安静地陪你待一会儿。".to_string()
        }
    }

    #[test]
    fn dialogue_engine_uses_provider_boundary_and_policy() {
        let engine = DialogueEngine::new(
            Box::new(LongReplyProvider),
            DialoguePolicy::with_max_chars(6),
        );

        let actions = engine.reply_to(
            "你在干嘛".to_string(),
            &IanState::default(),
            DialogueSource::UserBubble,
            MoodState::Happy,
            BondStateView::GettingCloser,
        );

        assert!(actions.iter().any(|action| matches!(
            action,
            IanAction::SpeechShow { text, .. } if text == "我正在安静地"
        )));
        assert!(actions.iter().any(|action| matches!(
            action,
            IanAction::AnimationPlay { name, .. } if name == "happy"
        )));
    }
}
