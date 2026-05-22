use crate::{
    domain::{bond::BondStateView, mood::MoodState},
    protocol::{IanAction, IanState},
};
use std::sync::Mutex;

use super::{
    dialogue_policy::DialoguePolicy,
    providers::{demo::DemoDialogueProvider, DialogueContext, DialogueProvider, DialogueSource},
};

pub struct DialogueEngine {
    provider: Box<dyn DialogueProvider>,
    policy: DialoguePolicy,
    short_context: Mutex<ShortReplyContext>,
}

#[derive(Debug, Default)]
struct ShortReplyContext {
    last_intent: Option<ReplyIntent>,
    last_reply: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReplyIntent {
    Greeting,
    Water,
    Rest,
    Night,
    Companionship,
}

impl Default for DialogueEngine {
    fn default() -> Self {
        Self {
            provider: Box::<DemoDialogueProvider>::default(),
            policy: DialoguePolicy::default(),
            short_context: Mutex::new(ShortReplyContext::default()),
        }
    }
}

impl DialogueEngine {
    pub fn new(provider: Box<dyn DialogueProvider>, policy: DialoguePolicy) -> Self {
        Self {
            provider,
            policy,
            short_context: Mutex::new(ShortReplyContext::default()),
        }
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
            day_phase: state.day_phase.clone(),
            interaction_count: 0,
            recent_activity_level: "none".to_string(),
        };
        let intent = classify_reply_intent(&text, &context);
        let reply = self.remembered_reply(
            intent,
            self.policy.apply(self.provider.reply(&text, &context)),
        );

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

    fn remembered_reply(&self, intent: ReplyIntent, reply: String) -> String {
        let Ok(mut context) = self.short_context.lock() else {
            return reply;
        };

        let resolved = if context.last_intent == Some(intent)
            && context.last_reply.as_deref() == Some(reply.as_str())
        {
            alternate_reply(intent, &reply).to_string()
        } else {
            reply
        };

        context.last_intent = Some(intent);
        context.last_reply = Some(resolved.clone());
        resolved
    }
}

fn classify_reply_intent(text: &str, context: &DialogueContext) -> ReplyIntent {
    let lowered = text.to_lowercase();
    if context.day_phase == "night" {
        ReplyIntent::Night
    } else if lowered.contains("water") || text.contains('水') {
        ReplyIntent::Water
    } else if lowered.contains("sleep") || text.contains('困') {
        ReplyIntent::Rest
    } else if lowered.contains("hello")
        || lowered.contains("hi")
        || text.contains('你')
        || text.contains('好')
        || text.contains("在吗")
    {
        ReplyIntent::Greeting
    } else {
        ReplyIntent::Companionship
    }
}

fn alternate_reply(intent: ReplyIntent, current: &str) -> &'static str {
    match intent {
        ReplyIntent::Water if current != "我也想喝一口。" => "我也想喝一口。",
        ReplyIntent::Rest if current != "慢慢来，我陪着。" => "慢慢来，我陪着。",
        ReplyIntent::Night if current != "灯暗一点也好。" => "灯暗一点也好。",
        ReplyIntent::Greeting if current != "嗯，我在旁边。" => "嗯，我在旁边。",
        _ if current != "贴近一点点。" => "贴近一点点。",
        _ => "我在这儿。",
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
            assert_eq!(context.day_phase, "day");
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

    #[test]
    fn demo_dialogue_uses_life_context_without_network() {
        let engine = DialogueEngine::default();
        let mut state = IanState::default();
        state.current_animation = "sleep".to_string();
        state.day_phase = "night".to_string();

        let actions = engine.reply_to(
            "你在吗".to_string(),
            &state,
            DialogueSource::UserBubble,
            MoodState::Calm,
            BondStateView::Familiar,
        );

        assert!(actions.iter().any(|action| matches!(
            action,
            IanAction::SpeechShow { text, .. } if text.contains("小声")
        )));
    }

    #[test]
    fn repeated_demo_intents_do_not_return_the_same_short_reply() {
        let engine = DialogueEngine::default();
        let state = IanState::default();

        let first = engine.reply_to(
            "你好".to_string(),
            &state,
            DialogueSource::UserBubble,
            MoodState::Calm,
            BondStateView::New,
        );
        let second = engine.reply_to(
            "还在吗".to_string(),
            &state,
            DialogueSource::UserBubble,
            MoodState::Calm,
            BondStateView::New,
        );

        assert_ne!(speech_text(&first), speech_text(&second));
    }

    fn speech_text(actions: &[IanAction]) -> String {
        actions
            .iter()
            .find_map(|action| match action {
                IanAction::SpeechShow { text, .. } => Some(text.clone()),
                _ => None,
            })
            .expect("speech action")
    }
}
