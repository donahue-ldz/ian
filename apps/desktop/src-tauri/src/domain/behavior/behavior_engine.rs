use crate::{
    core::scheduler::BehaviorScheduler,
    protocol::{IanAction, IanEvent, IanState, MovementSpeed, Position},
};

use super::{
    behavior_policy::BehaviorPolicy, developer_rhythm_policy::DeveloperRhythmPolicy,
    movement_boundary_policy::MovementBoundaryPolicy,
};

pub struct BehaviorEngine {
    policy: BehaviorPolicy,
    scheduler: BehaviorScheduler,
    movement_boundary: MovementBoundaryPolicy,
    affection_count: std::sync::Mutex<u32>,
    developer_rhythm: std::sync::Mutex<DeveloperRhythmPolicy>,
}

impl Default for BehaviorEngine {
    fn default() -> Self {
        Self {
            policy: BehaviorPolicy::default(),
            scheduler: BehaviorScheduler::default(),
            movement_boundary: MovementBoundaryPolicy::default(),
            affection_count: std::sync::Mutex::new(0),
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
            IanEvent::MouseClick { .. } => self.actions_for_click(state),
            IanEvent::MouseDoubleClick { .. } => self.actions_for_run_around(state),
            IanEvent::MouseNear { .. } => vec![IanAction::AnimationPlay {
                name: "happy".to_string(),
                looped: false,
            }],
            IanEvent::MouseLeave { .. } => vec![IanAction::AnimationPlay {
                name: "idle".to_string(),
                looped: true,
            }],
            IanEvent::MouseDragEnd { x, y } => vec![IanAction::MovementMoveTo {
                x: *x,
                y: *y,
                speed: MovementSpeed::Normal,
            }],
            IanEvent::TimeTick { now_ms } => self.actions_for_tick(*now_ms, state),
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

    fn actions_for_click(&self, state: &IanState) -> Vec<IanAction> {
        if state.current_animation == "sleep" {
            return vec![IanAction::AnimationPlay {
                name: "idle".to_string(),
                looped: true,
            }];
        }

        let interaction_count = self
            .affection_count
            .lock()
            .map(|mut count| {
                *count += 1;
                *count
            })
            .unwrap_or(1);

        vec![
            IanAction::BubbleOpen,
            IanAction::SpeechShow {
                text: self
                    .policy
                    .affectionate_phrase(interaction_count)
                    .to_string(),
                mood: Some("calm".to_string()),
                duration_ms: Some(2400),
            },
            IanAction::AnimationPlay {
                name: "happy".to_string(),
                looped: false,
            },
        ]
    }

    fn actions_for_run_around(&self, state: &IanState) -> Vec<IanAction> {
        let origin = state.position.clone();
        let mode = &state.behavior_mode;
        let first = self.movement_boundary.constrain_target(
            origin.clone(),
            Position {
                x: origin.x + 90.0,
                y: origin.y,
            },
            mode,
        );
        let second = self.movement_boundary.constrain_target(
            first.clone(),
            Position {
                x: origin.x + 40.0,
                y: origin.y - 60.0,
            },
            mode,
        );

        vec![
            IanAction::BehaviorRunAround {
                duration_ms: self.policy.run_around_duration_ms(),
            },
            IanAction::AnimationPlay {
                name: "run".to_string(),
                looped: true,
            },
            IanAction::MovementMoveTo {
                x: first.x,
                y: first.y,
                speed: MovementSpeed::Fast,
            },
            IanAction::MovementMoveTo {
                x: second.x,
                y: second.y,
                speed: MovementSpeed::Fast,
            },
            IanAction::MovementMoveTo {
                x: state.home_anchor.x,
                y: state.home_anchor.y,
                speed: MovementSpeed::Fast,
            },
            IanAction::AnimationPlay {
                name: "idle".to_string(),
                looped: true,
            },
        ]
    }

    fn actions_for_tick(&self, now_ms: i64, state: &IanState) -> Vec<IanAction> {
        let mut actions: Vec<IanAction> = self
            .scheduler
            .action_for_tick(now_ms, state)
            .into_iter()
            .collect();

        if state.current_animation == "run"
            || matches!(state.behavior_mode, crate::protocol::BehaviorMode::Quiet)
        {
            return actions;
        }

        let second = now_ms.div_euclid(1000);
        if second > 0 && second % 45 == 0 {
            let target = self.movement_boundary.constrain_target(
                state.position.clone(),
                Position {
                    x: state.position.x + 32.0,
                    y: state.position.y + 18.0,
                },
                &state.behavior_mode,
            );
            actions.push(IanAction::MovementMoveTo {
                x: target.x,
                y: target.y,
                speed: MovementSpeed::Slow,
            });
        }

        actions
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
    fn mouse_leave_returns_attention_to_idle_without_global_tracking() {
        let actions = BehaviorEngine::default().decide(
            &IanEvent::MouseLeave { x: 32.0, y: 48.0 },
            &IanState::default(),
        );

        assert!(actions.iter().any(|action| matches!(
            action,
            IanAction::AnimationPlay { name, looped } if name == "idle" && *looped
        )));
    }

    #[test]
    fn double_click_emits_run_animation_path_and_anchor_return() {
        let engine = BehaviorEngine::default();
        let mut state = IanState::default();
        state.position.x = 100.0;
        state.position.y = 100.0;
        state.home_anchor.x = 100.0;
        state.home_anchor.y = 100.0;

        let actions = engine.decide(&IanEvent::MouseDoubleClick { x: 100.0, y: 100.0 }, &state);

        assert!(matches!(
            actions.first(),
            Some(IanAction::BehaviorRunAround { .. })
        ));
        assert!(actions.iter().any(|action| matches!(
            action,
            IanAction::AnimationPlay { name, looped } if name == "run" && *looped
        )));

        let movement_count = actions
            .iter()
            .filter(|action| matches!(action, IanAction::MovementMoveTo { .. }))
            .count();
        assert!((2..=4).contains(&movement_count));
        assert!(matches!(
            actions.iter().rev().find(|action| matches!(action, IanAction::AnimationPlay { .. })),
            Some(IanAction::AnimationPlay { name, looped }) if name == "idle" && *looped
        ));
        assert!(matches!(
            actions.iter().rev().find(|action| matches!(action, IanAction::MovementMoveTo { .. })),
            Some(IanAction::MovementMoveTo { x, y, .. }) if *x == state.home_anchor.x && *y == state.home_anchor.y
        ));
    }

    #[test]
    fn time_tick_idle_roam_moves_without_interrupting_run() {
        let engine = BehaviorEngine::default();
        let mut state = IanState::default();
        state.position.x = 80.0;
        state.position.y = 80.0;

        let idle_actions = engine.decide(&IanEvent::TimeTick { now_ms: 45_000 }, &state);
        assert!(idle_actions
            .iter()
            .any(|action| matches!(action, IanAction::MovementMoveTo { .. })));

        state.current_animation = "run".to_string();
        let running_actions = engine.decide(&IanEvent::TimeTick { now_ms: 90_000 }, &state);
        assert!(!running_actions
            .iter()
            .any(|action| matches!(action, IanAction::MovementMoveTo { .. })));
        assert!(!running_actions.iter().any(
            |action| matches!(action, IanAction::AnimationPlay { name, .. } if name == "sleep")
        ));
    }

    #[test]
    fn click_wakes_a_sleeping_ian_before_showing_affection() {
        let engine = BehaviorEngine::default();
        let mut state = IanState::default();
        state.current_animation = "sleep".to_string();

        let actions = engine.decide(&IanEvent::MouseClick { x: 1.0, y: 1.0 }, &state);

        assert!(matches!(
            actions.iter().find(|action| matches!(action, IanAction::AnimationPlay { .. })),
            Some(IanAction::AnimationPlay { name, looped }) if name == "idle" && *looped
        ));
    }

    #[test]
    fn repeated_clicks_make_affection_visible_without_exposing_scores() {
        let engine = BehaviorEngine::default();
        let state = IanState::default();

        let _ = engine.decide(&IanEvent::MouseClick { x: 1.0, y: 1.0 }, &state);
        let second = engine.decide(&IanEvent::MouseClick { x: 1.0, y: 1.0 }, &state);

        assert!(second.iter().any(|action| matches!(
            action,
            IanAction::SpeechShow { text, .. } if text == "再摸摸也可以。"
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
