use crate::{
    core::scheduler::BehaviorScheduler,
    protocol::{IanAction, IanEvent, IanState, MovementSpeed, Position},
};

use super::{
    behavior_policy::BehaviorPolicy, day_phase_policy::minute_of_day_from_epoch_ms,
    developer_rhythm_policy::DeveloperRhythmPolicy,
    movement_boundary_policy::MovementBoundaryPolicy,
};

pub struct BehaviorEngine {
    policy: BehaviorPolicy,
    scheduler: BehaviorScheduler,
    movement_boundary: MovementBoundaryPolicy,
    affection_count: std::sync::Mutex<u32>,
    attention_available_after_ms: std::sync::Mutex<i64>,
    developer_rhythm: std::sync::Mutex<DeveloperRhythmPolicy>,
}

impl Default for BehaviorEngine {
    fn default() -> Self {
        Self {
            policy: BehaviorPolicy::default(),
            scheduler: BehaviorScheduler::default(),
            movement_boundary: MovementBoundaryPolicy::default(),
            affection_count: std::sync::Mutex::new(0),
            attention_available_after_ms: std::sync::Mutex::new(0),
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
            IanEvent::MouseNear { now_ms, .. } => self.actions_for_attention(*now_ms),
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
            IanEvent::MouseDragStart { .. }
            | IanEvent::DialogueUserMessage { .. }
            | IanEvent::BubbleInputStarted
            | IanEvent::BubbleInputEnded => vec![],
        }
    }

    fn actions_for_click(&self, state: &IanState) -> Vec<IanAction> {
        if state.current_animation == "run" {
            return Vec::new();
        }

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

        if interaction_count == 3 {
            return vec![IanAction::AnimationPlay {
                name: "happy".to_string(),
                looped: false,
            }];
        }

        if interaction_count >= 4 {
            return vec![
                IanAction::SpeechShow {
                    text: "有点痒，我挪一下。".to_string(),
                    mood: Some("calm".to_string()),
                    duration_ms: Some(1800),
                },
                IanAction::MovementMoveTo {
                    x: state.position.x + 24.0,
                    y: state.position.y,
                    speed: MovementSpeed::Slow,
                },
            ];
        }

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

    fn actions_for_attention(&self, now_ms: i64) -> Vec<IanAction> {
        let Ok(mut available_after_ms) = self.attention_available_after_ms.lock() else {
            return Vec::new();
        };

        if now_ms < *available_after_ms {
            return Vec::new();
        }

        *available_after_ms = now_ms + self.policy.attention_cooldown_ms();
        vec![IanAction::AnimationPlay {
            name: "happy".to_string(),
            looped: false,
        }]
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
        if is_user_interaction_active(state) {
            return Vec::new();
        }

        if state
            .quiet_hours
            .is_active_at_minute(minute_of_day_from_epoch_ms(now_ms))
        {
            return vec![IanAction::AnimationPlay {
                name: "idle".to_string(),
                looped: true,
            }];
        }

        let mut actions: Vec<IanAction> = self
            .scheduler
            .action_for_tick(now_ms, state)
            .into_iter()
            .collect();

        if state.current_animation == "run"
            || matches!(state.behavior_mode, crate::protocol::BehaviorMode::Quiet)
            || state.day_phase == "night"
        {
            return actions;
        }

        if actions.iter().any(
            |action| matches!(action, IanAction::AnimationPlay { name, .. } if name == "sleep"),
        ) {
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

fn is_user_interaction_active(state: &IanState) -> bool {
    state.is_dragging || state.is_bubble_input_active || state.current_animation == "run"
}

#[cfg(test)]
mod tests {
    use crate::protocol::{IanAction, IanEvent, IanState};

    use super::BehaviorEngine;

    #[test]
    fn mouse_near_returns_a_light_happy_reaction() {
        let actions = BehaviorEngine::default().decide(
            &IanEvent::MouseNear {
                x: 32.0,
                y: 48.0,
                now_ms: 1_000,
            },
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
    fn mouse_near_attention_stays_local_to_ian_window() {
        let engine = BehaviorEngine::default();
        let state = IanState::default();

        let first = engine.decide(
            &IanEvent::MouseNear {
                x: 32.0,
                y: 48.0,
                now_ms: 1_000,
            },
            &state,
        );

        assert!(plays_animation(&first, "happy"));
    }

    #[test]
    fn mouse_near_attention_respects_cooldown_and_recovers_after_window() {
        let engine = BehaviorEngine::default();
        let state = IanState::default();

        let first = engine.decide(
            &IanEvent::MouseNear {
                x: 32.0,
                y: 48.0,
                now_ms: 1_000,
            },
            &state,
        );
        let repeated = engine.decide(
            &IanEvent::MouseNear {
                x: 32.0,
                y: 48.0,
                now_ms: 1_500,
            },
            &state,
        );
        let after_cooldown = engine.decide(
            &IanEvent::MouseNear {
                x: 32.0,
                y: 48.0,
                now_ms: 7_500,
            },
            &state,
        );

        assert!(plays_animation(&first, "happy"));
        assert!(!plays_animation(&repeated, "happy"));
        assert!(plays_animation(&after_cooldown, "happy"));
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
    fn time_tick_does_not_roam_or_sleep_while_run_is_active() {
        let engine = BehaviorEngine::default();
        let mut running = IanState::default();
        running.current_animation = "run".to_string();

        let running_actions = engine.decide(&IanEvent::TimeTick { now_ms: 90_000 }, &running);

        assert_no_autonomous_movement_or_sleep(&running_actions);
    }

    #[test]
    fn time_tick_does_not_roam_or_sleep_while_user_is_interacting() {
        let engine = BehaviorEngine::default();
        let mut dragging = IanState::default();
        dragging.is_dragging = true;
        let mut typing = IanState::default();
        typing.is_bubble_input_active = true;

        let dragging_actions = engine.decide(&IanEvent::TimeTick { now_ms: 90_000 }, &dragging);
        let typing_actions = engine.decide(&IanEvent::TimeTick { now_ms: 90_000 }, &typing);

        assert_no_autonomous_movement_or_sleep(&dragging_actions);
        assert_no_autonomous_movement_or_sleep(&typing_actions);
    }

    #[test]
    fn sleep_candidate_wins_over_lower_priority_roam() {
        let engine = BehaviorEngine::default();
        let state = IanState::default();

        let actions = engine.decide(&IanEvent::TimeTick { now_ms: 90_000 }, &state);

        assert!(plays_animation(&actions, "sleep"));
        assert!(!actions
            .iter()
            .any(|action| matches!(action, IanAction::MovementMoveTo { .. })));
    }

    #[test]
    fn quiet_hours_override_lively_autonomous_movement() {
        let engine = BehaviorEngine::default();
        let mut state = IanState::default();
        state.behavior_mode = crate::protocol::BehaviorMode::Lively;
        state.quiet_hours.enabled = true;
        state.quiet_hours.start_minute = 0;
        state.quiet_hours.end_minute = 24 * 60;

        let actions = engine.decide(&IanEvent::TimeTick { now_ms: 45_000 }, &state);

        assert!(!actions
            .iter()
            .any(|action| matches!(action, IanAction::MovementMoveTo { .. })));
        assert!(actions.iter().any(|action| matches!(
            action,
            IanAction::AnimationPlay { name, looped } if name == "idle" && *looped
        )));
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
    fn repeated_clicks_are_rate_limited_and_excessive_clicks_move_gently_away() {
        let engine = BehaviorEngine::default();
        let state = IanState::default();

        let first = engine.decide(&IanEvent::MouseClick { x: 1.0, y: 1.0 }, &state);
        let second = engine.decide(&IanEvent::MouseClick { x: 1.0, y: 1.0 }, &state);
        let third = engine.decide(&IanEvent::MouseClick { x: 1.0, y: 1.0 }, &state);
        let fourth = engine.decide(&IanEvent::MouseClick { x: 1.0, y: 1.0 }, &state);

        assert!(first
            .iter()
            .any(|action| matches!(action, IanAction::SpeechShow { .. })));
        assert!(second.iter().any(|action| matches!(
            action,
            IanAction::SpeechShow { text, .. } if text == "再摸摸也可以。"
        )));
        assert!(!third
            .iter()
            .any(|action| matches!(action, IanAction::SpeechShow { .. })));
        assert!(
            fourth.iter().any(|action| matches!(
                action,
                IanAction::SpeechShow { text, .. } if text == "有点痒，我挪一下。"
            )) || fourth
                .iter()
                .any(|action| matches!(action, IanAction::MovementMoveTo { .. }))
        );
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

    fn plays_animation(actions: &[IanAction], animation: &str) -> bool {
        actions.iter().any(|action| {
            matches!(
                action,
                IanAction::AnimationPlay { name, .. } if name == animation
            )
        })
    }

    fn assert_no_autonomous_movement_or_sleep(actions: &[IanAction]) {
        assert!(!actions.iter().any(|action| match action {
            IanAction::MovementMoveTo { .. } => true,
            IanAction::AnimationPlay { name, .. } => name == "sleep",
            _ => false,
        }));
    }
}
