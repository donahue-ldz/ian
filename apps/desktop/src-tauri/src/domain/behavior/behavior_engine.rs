use crate::protocol::{IanAction, IanEvent, IanState, MovementSpeed};

use super::behavior_policy::BehaviorPolicy;

pub struct BehaviorEngine {
    policy: BehaviorPolicy,
}

impl Default for BehaviorEngine {
    fn default() -> Self {
        Self {
            policy: BehaviorPolicy::default(),
        }
    }
}

impl BehaviorEngine {
    pub fn decide(&self, event: &IanEvent, state: &IanState) -> Vec<IanAction> {
        match event {
            IanEvent::AppStarted => vec![
                IanAction::AnimationPlay {
                    name: "idle".to_string(),
                    looped: true,
                },
                IanAction::StateSync {
                    state: state.clone(),
                },
            ],
            IanEvent::MouseClick { .. } => vec![
                IanAction::BubbleOpen,
                IanAction::SpeechShow {
                    text: self.policy.click_phrase().to_string(),
                    mood: Some("calm".to_string()),
                    duration_ms: Some(2400),
                },
                IanAction::AnimationPlay {
                    name: "happy".to_string(),
                    looped: false,
                },
            ],
            IanEvent::MouseDoubleClick { .. } => vec![IanAction::BehaviorRunAround {
                duration_ms: self.policy.run_around_duration_ms(),
            }],
            IanEvent::MouseDragEnd { x, y } => vec![IanAction::MovementMoveTo {
                x: *x,
                y: *y,
                speed: MovementSpeed::Normal,
            }],
            IanEvent::TimeTick { .. } => vec![IanAction::AnimationPlay {
                name: "idle".to_string(),
                looped: true,
            }],
            IanEvent::MouseDragStart { .. } | IanEvent::DialogueUserMessage { .. } => vec![],
        }
    }
}
