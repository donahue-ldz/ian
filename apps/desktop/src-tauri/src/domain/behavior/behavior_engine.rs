use crate::{
    core::scheduler::BehaviorScheduler,
    protocol::{IanAction, IanEvent, IanState, MovementSpeed},
};

use super::{behavior_policy::BehaviorPolicy, developer_rhythm_policy::DeveloperRhythmPolicy};

pub struct BehaviorEngine {
    policy: BehaviorPolicy,
    scheduler: BehaviorScheduler,
    developer_rhythm: std::sync::Mutex<DeveloperRhythmPolicy>,
}

impl Default for BehaviorEngine {
    fn default() -> Self {
        Self {
            policy: BehaviorPolicy::default(),
            scheduler: BehaviorScheduler::default(),
            developer_rhythm: std::sync::Mutex::new(DeveloperRhythmPolicy::default()),
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
            IanEvent::MouseNear { .. } => vec![IanAction::AnimationPlay {
                name: "happy".to_string(),
                looped: false,
            }],
            IanEvent::MouseDragEnd { x, y } => vec![IanAction::MovementMoveTo {
                x: *x,
                y: *y,
                speed: MovementSpeed::Normal,
            }],
            IanEvent::TimeTick { now_ms } => self
                .scheduler
                .action_for_tick(*now_ms, state)
                .into_iter()
                .collect(),
            IanEvent::DeveloperBuildTestSummary { .. }
            | IanEvent::DeveloperGitStatusChanged { .. } => self
                .developer_rhythm
                .lock()
                .map(|mut policy| policy.actions_for_event(event, 0))
                .unwrap_or_default(),
            IanEvent::KeyboardRhythm { .. } | IanEvent::ActiveAppPresence { .. } => vec![],
            IanEvent::MouseDragStart { .. } | IanEvent::DialogueUserMessage { .. } => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::protocol::{IanAction, IanEvent, IanState};

    use super::BehaviorEngine;

    #[test]
    fn mouse_near_returns_a_light_happy_reaction() {
        let actions = BehaviorEngine::default().decide(
            &IanEvent::MouseNear { x: 32.0, y: 48.0 },
            &IanState::default(),
        );

        assert!(actions.iter().any(|action| matches!(
            action,
            IanAction::AnimationPlay { name, looped } if name == "happy" && !looped
        )));
    }

    #[test]
    fn time_tick_returns_predictable_idle_micro_actions() {
        let engine = BehaviorEngine::default();
        let state = IanState::default();

        let walk_actions = engine.decide(&IanEvent::TimeTick { now_ms: 45_000 }, &state);
        let sleep_actions = engine.decide(&IanEvent::TimeTick { now_ms: 90_000 }, &state);

        assert!(walk_actions.iter().any(|action| matches!(
            action,
            IanAction::AnimationPlay { name, looped } if name == "walk" && !looped
        )));
        assert!(sleep_actions.iter().any(|action| matches!(
            action,
            IanAction::AnimationPlay { name, looped } if name == "sleep" && *looped
        )));
    }
}
