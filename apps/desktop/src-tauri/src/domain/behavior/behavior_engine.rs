use crate::{
    core::scheduler::BehaviorScheduler,
    protocol::{IanAction, IanEvent, IanState, MovementSpeed, PlayfulState, Position},
};

use super::{
    behavior_policy::{BehaviorPolicy, ControlledRandom},
    day_phase_policy::minute_of_day_from_epoch_ms,
    developer_rhythm_policy::DeveloperRhythmPolicy,
    life_rhythm_policy::{AbsenceReturnPolicy, DailyGreetingPolicy},
    moment_orchestrator::{IanMomentKind, MomentOrchestrator},
    momentary_life_state::MomentaryLifeState,
    movement_boundary_policy::MovementBoundaryPolicy,
};

pub struct BehaviorEngine {
    policy: BehaviorPolicy,
    scheduler: BehaviorScheduler,
    movement_boundary: MovementBoundaryPolicy,
    affection_count: std::sync::Mutex<u32>,
    attention_available_after_ms: std::sync::Mutex<i64>,
    developer_rhythm: std::sync::Mutex<DeveloperRhythmPolicy>,
    momentary_life: std::sync::Mutex<MomentaryLifeState>,
    daily_greeting: std::sync::Mutex<DailyGreetingPolicy>,
    absence_return: std::sync::Mutex<AbsenceReturnPolicy>,
    moments: std::sync::Mutex<MomentOrchestrator>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TouchReaction {
    GentleClick,
    RapidPause,
    OverstimulatedNudge,
    DragStart,
    DragEnd,
    RunAround,
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
            momentary_life: std::sync::Mutex::new(MomentaryLifeState::default()),
            daily_greeting: std::sync::Mutex::new(DailyGreetingPolicy::default()),
            absence_return: std::sync::Mutex::new(AbsenceReturnPolicy::default()),
            moments: std::sync::Mutex::new(MomentOrchestrator::default()),
        }
    }
}

impl BehaviorEngine {
    pub fn decide(&self, event: &IanEvent, state: &IanState) -> Vec<IanAction> {
        self.record_momentary_event(event);

        let actions = match event {
            IanEvent::AppStarted => vec![
                IanAction::AnimationPlay {
                    name: "happy".to_string(),
                    looped: false,
                },
                IanAction::EffectPlay {
                    name: "sparkle_pop".to_string(),
                    intensity: "low".to_string(),
                    duration_ms: 900,
                },
                IanAction::BubbleOpen,
                IanAction::SpeechShow {
                    text: "醒啦。".to_string(),
                    mood: Some("calm".to_string()),
                    duration_ms: Some(1600),
                },
            ],
            IanEvent::MouseClick { .. } => self.actions_for_click(state),
            IanEvent::MouseDoubleClick { .. } => self.actions_for_run_around(state),
            IanEvent::MouseNear { now_ms, .. } => self.actions_for_attention(*now_ms, state),
            IanEvent::MouseLeave { .. } => vec![IanAction::AnimationPlay {
                name: "idle".to_string(),
                looped: true,
            }],
            IanEvent::MouseDragStart { .. } => {
                self.actions_for_touch_reaction(TouchReaction::DragStart, state, None, None)
            }
            IanEvent::MouseDragEnd { x, y } => self.actions_for_touch_reaction(
                TouchReaction::DragEnd,
                state,
                Some(Position { x: *x, y: *y }),
                None,
            ),
            IanEvent::MouseChaseCandidate { x, y, now_ms } => {
                self.actions_for_pointer_chase(*now_ms, Position { x: *x, y: *y }, state)
            }
            IanEvent::SystemShortcutTriggered { action, now_ms } if action == "find_ian" => {
                self.actions_for_find_ian(*now_ms, state)
            }
            IanEvent::SystemShortcutTriggered { .. } => vec![],
            IanEvent::MomentDebugTrigger { kind, now_ms } => {
                self.actions_for_debug_moment(kind, *now_ms, state)
            }
            IanEvent::TimeTick { now_ms } => self.actions_for_tick(*now_ms, state),
            IanEvent::ScreenBounds { .. } => vec![],
            IanEvent::DeveloperBuildTestSummary { .. }
            | IanEvent::DeveloperGitStatusChanged { .. } => self
                .developer_rhythm
                .lock()
                .map(|mut policy| {
                    policy.actions_for_event(event, chrono::Utc::now().timestamp_millis(), state)
                })
                .unwrap_or_default(),
            IanEvent::KeyboardRhythm { .. } | IanEvent::ActiveAppPresence { .. } => vec![],
            IanEvent::DialogueUserMessage { .. }
            | IanEvent::BubbleInputStarted
            | IanEvent::BubbleInputEnded => vec![],
        };

        self.record_momentary_actions(&actions);
        actions
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

        if let Some(phrase) = self.recent_wake_phrase() {
            return self.actions_for_affection_phrase(phrase.to_string());
        }

        if interaction_count == 3 {
            return self.actions_for_touch_reaction(
                TouchReaction::RapidPause,
                state,
                None,
                Some(interaction_count),
            );
        }

        if interaction_count >= 4 {
            return self.actions_for_touch_reaction(
                TouchReaction::OverstimulatedNudge,
                state,
                None,
                Some(interaction_count),
            );
        }

        self.actions_for_touch_reaction(
            TouchReaction::GentleClick,
            state,
            None,
            Some(interaction_count),
        )
    }

    fn actions_for_find_ian(&self, now_ms: i64, state: &IanState) -> Vec<IanAction> {
        let decision = self.moment_decision(IanMomentKind::FindIanEntrance, now_ms, state);
        let can_move = decision.triggered();
        let mut actions = vec![
            decision.diagnostic(now_ms),
            IanAction::BubbleOpen,
            IanAction::SpeechShow {
                text: "我在这儿。".to_string(),
                mood: Some("calm".to_string()),
                duration_ms: Some(2200),
            },
            IanAction::EffectPlay {
                name: "find_beacon".to_string(),
                intensity: "low".to_string(),
                duration_ms: 1_600,
            },
            IanAction::AnimationPlay {
                name: "find".to_string(),
                looped: false,
            },
            IanAction::AnimationPlay {
                name: "happy".to_string(),
                looped: false,
            },
        ];

        if can_move && !is_position_visible(state) {
            let target = find_ian_target(state);
            actions.insert(
                1,
                IanAction::MovementMoveTo {
                    x: target.x,
                    y: target.y,
                    speed: MovementSpeed::Fast,
                },
            );
        }

        actions
    }

    fn actions_for_attention(&self, now_ms: i64, state: &IanState) -> Vec<IanAction> {
        if is_user_interaction_active(state) {
            return Vec::new();
        }

        let Ok(mut available_after_ms) = self.attention_available_after_ms.lock() else {
            return Vec::new();
        };

        if now_ms < *available_after_ms {
            return Vec::new();
        }

        *available_after_ms = now_ms + self.policy.attention_cooldown_ms();
        let decision = self.moment_decision(IanMomentKind::PointerCuriosity, now_ms, state);
        if !decision.triggered() {
            return vec![decision.diagnostic(now_ms)];
        }

        vec![
            decision.diagnostic(now_ms),
            IanAction::AnimationPlay {
                name: "wave".to_string(),
                looped: false,
            },
            IanAction::AnimationPlay {
                name: "happy".to_string(),
                looped: false,
            },
            IanAction::EffectPlay {
                name: "sparkle_pop".to_string(),
                intensity: "low".to_string(),
                duration_ms: 700,
            },
            IanAction::SpeechShow {
                text: "看到你啦。".to_string(),
                mood: Some("calm".to_string()),
                duration_ms: Some(1200),
            },
        ]
    }

    fn actions_for_run_around(&self, state: &IanState) -> Vec<IanAction> {
        let _reaction = TouchReaction::RunAround;
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
            IanAction::PlayfulStateSet {
                state: PlayfulState::Settling,
                until_ms: Some(
                    chrono::Utc::now().timestamp_millis()
                        + self.policy.run_around_duration_ms() as i64
                        + 900,
                ),
            },
            IanAction::EffectPlay {
                name: "blush_puff".to_string(),
                intensity: "low".to_string(),
                duration_ms: 800,
            },
            IanAction::AnimationPlay {
                name: "idle".to_string(),
                looped: true,
            },
        ]
    }

    fn actions_for_touch_reaction(
        &self,
        reaction: TouchReaction,
        state: &IanState,
        target: Option<Position>,
        interaction_count: Option<u32>,
    ) -> Vec<IanAction> {
        match reaction {
            TouchReaction::GentleClick => self.actions_for_affection_phrase(
                self.policy
                    .affectionate_phrase(interaction_count.unwrap_or(1))
                    .to_string(),
            ),
            TouchReaction::RapidPause => vec![
                IanAction::AnimationPlay {
                    name: "happy".to_string(),
                    looped: false,
                },
                IanAction::EffectPlay {
                    name: "sparkle_pop".to_string(),
                    intensity: "low".to_string(),
                    duration_ms: 700,
                },
            ],
            TouchReaction::OverstimulatedNudge => vec![
                IanAction::SpeechShow {
                    text: "有点痒，我挪一下。".to_string(),
                    mood: Some("calm".to_string()),
                    duration_ms: Some(1800),
                },
                IanAction::EffectPlay {
                    name: "blush_puff".to_string(),
                    intensity: "low".to_string(),
                    duration_ms: 800,
                },
                IanAction::MovementMoveTo {
                    x: state.position.x + 24.0,
                    y: state.position.y,
                    speed: MovementSpeed::Slow,
                },
            ],
            TouchReaction::DragStart => vec![
                self.moment_decision(
                    IanMomentKind::DragCarry,
                    chrono::Utc::now().timestamp_millis(),
                    state,
                )
                .diagnostic(chrono::Utc::now().timestamp_millis()),
                IanAction::AppearanceScaleTo {
                    scale: 0.92,
                    duration_ms: 180,
                },
                IanAction::AnimationPlay {
                    name: "affection".to_string(),
                    looped: false,
                },
                IanAction::AnimationPlay {
                    name: "happy".to_string(),
                    looped: false,
                },
                IanAction::EffectPlay {
                    name: "sparkle_pop".to_string(),
                    intensity: "low".to_string(),
                    duration_ms: 700,
                },
                IanAction::SpeechShow {
                    text: "抱起来啦。".to_string(),
                    mood: Some("calm".to_string()),
                    duration_ms: Some(1200),
                },
            ],
            TouchReaction::DragEnd => {
                let target = target.unwrap_or_else(|| state.position.clone());
                let distance = ((target.x - state.position.x).powi(2)
                    + (target.y - state.position.y).powi(2))
                .sqrt();
                let mut actions = vec![
                    IanAction::MovementMoveTo {
                        x: target.x,
                        y: target.y,
                        speed: MovementSpeed::Normal,
                    },
                    IanAction::EffectPlay {
                        name: "blush_puff".to_string(),
                        intensity: "low".to_string(),
                        duration_ms: 800,
                    },
                    IanAction::AnimationPlay {
                        name: "happy".to_string(),
                        looped: false,
                    },
                    IanAction::SpeechShow {
                        text: "放这里。".to_string(),
                        mood: Some("calm".to_string()),
                        duration_ms: Some(1600),
                    },
                ];

                if distance >= 48.0 {
                    actions.push(
                        self.moment_decision(
                            IanMomentKind::DropSettle,
                            chrono::Utc::now().timestamp_millis(),
                            state,
                        )
                        .diagnostic(chrono::Utc::now().timestamp_millis()),
                    );
                }

                actions
            }
            TouchReaction::RunAround => self.actions_for_run_around(state),
        }
    }

    fn actions_for_tick(&self, now_ms: i64, state: &IanState) -> Vec<IanAction> {
        if is_user_interaction_active(state) {
            return Vec::new();
        }

        if is_active_settling_state(now_ms, state) {
            return Vec::new();
        }

        if state.do_not_disturb
            || state
                .quiet_hours
                .is_active_at_minute(minute_of_day_from_epoch_ms(now_ms))
            || state
                .active_app_category
                .as_deref()
                .map(|category| {
                    self.policy
                        .should_reduce_disturbance_for_app_category(category)
                })
                .unwrap_or(false)
        {
            return vec![IanAction::AnimationPlay {
                name: "idle".to_string(),
                looped: true,
            }];
        }

        if let Some(actions) = self.actions_for_life_rhythm_tick(now_ms, state) {
            return actions;
        }

        if is_reduced_motion_enabled(state) {
            return self
                .scheduler
                .action_for_tick(now_ms, state)
                .into_iter()
                .collect();
        }

        if let Some(actions) = self.actions_for_rare_idle_moment(now_ms, state) {
            return actions;
        }

        if let Some(actions) = self.actions_for_perimeter_patrol_tick(now_ms, state) {
            return actions;
        }

        if let Some(actions) = self.actions_for_tantrum_tick(now_ms, state) {
            return actions;
        }

        if let Some(actions) = self.actions_for_playful_tick(now_ms, state) {
            return actions;
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

        if self
            .policy
            .should_emit_micro_motion(now_ms, &state.behavior_mode)
        {
            actions.extend(self.actions_for_micro_motion());
        }

        let second = now_ms.div_euclid(1000);
        let movement_profile = self.policy.movement_profile(&state.behavior_mode);
        let should_roam = movement_profile
            .roam_interval_secs
            .map(|interval| second > 0 && second % interval == 0)
            .unwrap_or(false);

        if should_roam {
            actions.extend(self.actions_for_visible_roam_path(state));
        }

        actions
    }

    fn actions_for_life_rhythm_tick(
        &self,
        now_ms: i64,
        state: &IanState,
    ) -> Option<Vec<IanAction>> {
        const DAY_MS: i64 = 24 * 60 * 60 * 1000;
        let day_index = now_ms.div_euclid(DAY_MS);

        if state.last_user_interaction_ms > 0 {
            if let Some(phrase) = self
                .absence_return
                .lock()
                .ok()
                .and_then(|mut policy| policy.reaction(state.last_user_interaction_ms, now_ms))
            {
                return Some(self.actions_for_life_phrase(phrase.to_string()));
            }
        }

        if day_index > 0 {
            if let Some(phrase) = self
                .daily_greeting
                .lock()
                .ok()
                .and_then(|mut policy| policy.greeting_for_day(day_index, state))
            {
                return Some(self.actions_for_life_phrase(phrase.to_string()));
            }
        }

        None
    }

    fn actions_for_life_phrase(&self, text: String) -> Vec<IanAction> {
        vec![
            IanAction::BubbleOpen,
            IanAction::SpeechShow {
                text,
                mood: Some("calm".to_string()),
                duration_ms: Some(2200),
            },
            IanAction::AnimationPlay {
                name: "happy".to_string(),
                looped: false,
            },
        ]
    }

    fn actions_for_rare_idle_moment(
        &self,
        now_ms: i64,
        state: &IanState,
    ) -> Option<Vec<IanAction>> {
        let second = now_ms.div_euclid(1000);
        if second < 3_600 || second.rem_euclid(3_600) >= 15 {
            return None;
        }

        let decision = self.moment_decision(IanMomentKind::RareIdleSurprise, now_ms, state);
        if !decision.triggered() {
            return Some(vec![decision.diagnostic(now_ms)]);
        }

        Some(vec![
            decision.diagnostic(now_ms),
            IanAction::AnimationPlay {
                name: "wave".to_string(),
                looped: false,
            },
            IanAction::EffectPlay {
                name: "tail_wag".to_string(),
                intensity: "low".to_string(),
                duration_ms: 900,
            },
            IanAction::AnimationPlay {
                name: "idle".to_string(),
                looped: true,
            },
        ])
    }

    fn actions_for_debug_moment(
        &self,
        kind: &str,
        now_ms: i64,
        state: &IanState,
    ) -> Vec<IanAction> {
        let Some(kind) = IanMomentKind::from_key(kind) else {
            return vec![self.playful_diagnostic(
                now_ms,
                "moment_debug",
                "blocked_unknown_kind",
                None,
                None,
            )];
        };

        if !state.diagnostics_enabled {
            return vec![self.playful_diagnostic(
                now_ms,
                kind.key(),
                "blocked_diagnostics_disabled",
                Some(kind.key()),
                None,
            )];
        }

        let diagnostic = self.playful_diagnostic(
            now_ms,
            kind.key(),
            "diagnostic_triggered",
            Some(kind.key()),
            Some(&format!("{}_sequence", kind.key())),
        );

        match kind {
            IanMomentKind::FindIanEntrance => {
                let mut actions = vec![
                    diagnostic,
                    IanAction::BubbleOpen,
                    IanAction::SpeechShow {
                        text: "我在这儿。".to_string(),
                        mood: Some("calm".to_string()),
                        duration_ms: Some(1800),
                    },
                    IanAction::EffectPlay {
                        name: "find_beacon".to_string(),
                        intensity: "low".to_string(),
                        duration_ms: 1200,
                    },
                    IanAction::AnimationPlay {
                        name: "find".to_string(),
                        looped: false,
                    },
                ];
                if !is_reduced_motion_enabled(state) {
                    let target = find_ian_target(state);
                    actions.insert(
                        1,
                        IanAction::MovementMoveTo {
                            x: target.x,
                            y: target.y,
                            speed: MovementSpeed::Fast,
                        },
                    );
                }
                actions
            }
            IanMomentKind::PointerCuriosity => {
                if is_reduced_motion_enabled(state) {
                    return vec![
                        diagnostic,
                        IanAction::SpeechShow {
                            text: "看到你啦。".to_string(),
                            mood: Some("calm".to_string()),
                            duration_ms: Some(1200),
                        },
                    ];
                }

                vec![
                    diagnostic,
                    IanAction::AnimationPlay {
                        name: "wave".to_string(),
                        looped: false,
                    },
                    IanAction::MovementMoveTo {
                        x: state.position.x + 36.0,
                        y: state.position.y,
                        speed: MovementSpeed::Slow,
                    },
                    IanAction::EffectPlay {
                        name: "sparkle_pop".to_string(),
                        intensity: "low".to_string(),
                        duration_ms: 600,
                    },
                    IanAction::AnimationPlay {
                        name: "idle".to_string(),
                        looped: true,
                    },
                ]
            }
            IanMomentKind::DragCarry => vec![
                diagnostic,
                IanAction::AppearanceScaleTo {
                    scale: 0.92,
                    duration_ms: 180,
                },
                IanAction::AnimationPlay {
                    name: "affection".to_string(),
                    looped: false,
                },
                IanAction::SpeechShow {
                    text: "抱起来啦。".to_string(),
                    mood: Some("calm".to_string()),
                    duration_ms: Some(1000),
                },
            ],
            IanMomentKind::DropSettle => vec![
                diagnostic,
                IanAction::MovementMoveTo {
                    x: state.position.x + 24.0,
                    y: state.position.y + 16.0,
                    speed: MovementSpeed::Normal,
                },
                IanAction::EffectPlay {
                    name: "blush_puff".to_string(),
                    intensity: "low".to_string(),
                    duration_ms: 700,
                },
                IanAction::SpeechShow {
                    text: "放这里。".to_string(),
                    mood: Some("calm".to_string()),
                    duration_ms: Some(1200),
                },
            ],
            IanMomentKind::RareIdleSurprise => {
                if is_reduced_motion_enabled(state) || state.do_not_disturb {
                    return vec![self.playful_diagnostic(
                        now_ms,
                        kind.key(),
                        if state.do_not_disturb {
                            "blocked_dnd"
                        } else {
                            "blocked_reduced_motion"
                        },
                        Some(kind.key()),
                        None,
                    )];
                }

                vec![
                    diagnostic,
                    IanAction::AnimationPlay {
                        name: "wave".to_string(),
                        looped: false,
                    },
                    IanAction::EffectPlay {
                        name: "tail_wag".to_string(),
                        intensity: "low".to_string(),
                        duration_ms: 800,
                    },
                    IanAction::AnimationPlay {
                        name: "idle".to_string(),
                        looped: true,
                    },
                ]
            }
            IanMomentKind::MemoryEcho => vec![
                diagnostic,
                IanAction::BubbleOpen,
                IanAction::SpeechShow {
                    text: "我记得你喜欢安静一点。".to_string(),
                    mood: Some("calm".to_string()),
                    duration_ms: Some(1800),
                },
            ],
        }
    }

    fn actions_for_perimeter_patrol_tick(
        &self,
        now_ms: i64,
        state: &IanState,
    ) -> Option<Vec<IanAction>> {
        const IDLE_THRESHOLD_MS: i64 = 60_000;
        const PATROL_MARGIN: f64 = 12.0;
        const PATROL_SCALE: f64 = 0.45;
        const DESKTOP_SURFACE_WIDTH: f64 = 220.0;
        const DESKTOP_SURFACE_HEIGHT: f64 = 220.0;
        const DESKTOP_SURFACE_OFFSET_X: f64 = 20.0;
        const DESKTOP_SURFACE_OFFSET_Y: f64 = 108.0;
        const PATROL_DURATION_MS: i64 = 180_000;
        const PATROL_VISUAL_WIDTH: f64 = DESKTOP_SURFACE_WIDTH * PATROL_SCALE;
        const PATROL_VISUAL_HEIGHT: f64 = DESKTOP_SURFACE_HEIGHT * PATROL_SCALE;
        const PATROL_VISUAL_OFFSET_X: f64 =
            DESKTOP_SURFACE_OFFSET_X + (DESKTOP_SURFACE_WIDTH - PATROL_VISUAL_WIDTH) / 2.0;
        const PATROL_VISUAL_OFFSET_Y: f64 =
            DESKTOP_SURFACE_OFFSET_Y + (DESKTOP_SURFACE_HEIGHT - PATROL_VISUAL_HEIGHT) / 2.0;

        if matches!(state.behavior_mode, crate::protocol::BehaviorMode::Quiet)
            || state.day_phase == "night"
            || state.current_animation == "run"
            || state.is_dragging
            || state.is_bubble_input_active
        {
            return None;
        }

        let bounds = state.screen_bounds.as_ref()?;
        if bounds.width < PATROL_VISUAL_WIDTH || bounds.height < PATROL_VISUAL_HEIGHT {
            return None;
        }

        if is_active_settling_state(now_ms, state) {
            return None;
        }

        let idle_for_ms = now_ms - state.last_user_interaction_ms;
        if idle_for_ms < IDLE_THRESHOLD_MS {
            return None;
        }

        let min_x = bounds.x + PATROL_MARGIN - PATROL_VISUAL_OFFSET_X;
        let min_y = bounds.y + PATROL_MARGIN - PATROL_VISUAL_OFFSET_Y;
        let max_x =
            bounds.x + bounds.width - PATROL_MARGIN - PATROL_VISUAL_WIDTH - PATROL_VISUAL_OFFSET_X;
        let max_y = bounds.y + bounds.height
            - PATROL_MARGIN
            - PATROL_VISUAL_HEIGHT
            - PATROL_VISUAL_OFFSET_Y;

        let corners = [
            Position { x: min_x, y: min_y },
            Position { x: max_x, y: min_y },
            Position { x: max_x, y: max_y },
            Position { x: min_x, y: max_y },
            Position { x: min_x, y: min_y },
        ];

        let mut actions = vec![
            IanAction::AppearanceScaleTo {
                scale: PATROL_SCALE,
                duration_ms: 500,
            },
            IanAction::AnimationPlay {
                name: "walk".to_string(),
                looped: true,
            },
            IanAction::PlayfulStateSet {
                state: PlayfulState::Settling,
                until_ms: Some(now_ms + PATROL_DURATION_MS),
            },
        ];

        actions.extend(corners.into_iter().map(|target| IanAction::MovementMoveTo {
            x: target.x,
            y: target.y,
            speed: MovementSpeed::Slow,
        }));

        Some(actions)
    }

    fn actions_for_micro_motion(&self) -> Vec<IanAction> {
        let (recently_affectionate, recently_dragged) = self
            .momentary_life
            .lock()
            .map(|life| (life.is_recently_affectionate(), life.is_recently_dragged()))
            .unwrap_or((false, false));
        let intensity = if recently_affectionate || recently_dragged {
            "medium"
        } else {
            "low"
        };

        vec![IanAction::EffectPlay {
            name: "tail_wag".to_string(),
            intensity: intensity.to_string(),
            duration_ms: 900,
        }]
    }

    fn actions_for_visible_roam_path(&self, state: &IanState) -> Vec<IanAction> {
        let movement_profile = self.policy.movement_profile(&state.behavior_mode);
        let first = self.movement_boundary.constrain_target(
            state.position.clone(),
            Position {
                x: state.position.x + movement_profile.roam_offset_x,
                y: state.position.y + movement_profile.roam_offset_y,
            },
            &state.behavior_mode,
        );
        let second = self.movement_boundary.constrain_target(
            first.clone(),
            Position {
                x: first.x - movement_profile.roam_offset_x * 0.45,
                y: first.y + movement_profile.roam_offset_y * 0.65,
            },
            &state.behavior_mode,
        );

        vec![
            IanAction::AnimationPlay {
                name: "walk".to_string(),
                looped: true,
            },
            IanAction::MovementMoveTo {
                x: first.x,
                y: first.y,
                speed: MovementSpeed::Normal,
            },
            IanAction::MovementMoveTo {
                x: second.x,
                y: second.y,
                speed: MovementSpeed::Slow,
            },
            IanAction::AnimationPlay {
                name: "idle".to_string(),
                looped: true,
            },
        ]
    }

    fn actions_for_tantrum_tick(&self, now_ms: i64, state: &IanState) -> Option<Vec<IanAction>> {
        if !matches!(state.behavior_mode, crate::protocol::BehaviorMode::Lively)
            || !matches!(state.playful_energy, crate::protocol::PlayfulEnergy::High)
        {
            return None;
        }

        let second = now_ms.div_euclid(1000);
        if second < 300 || second.rem_euclid(300) >= 15 {
            return None;
        }

        if matches!(
            state.playful_state,
            PlayfulState::Zooming | PlayfulState::CoolingDown | PlayfulState::WarmingUp
        ) && state
            .playful_state_until_ms
            .map(|until| now_ms < until)
            .unwrap_or(true)
        {
            return Some(vec![self.playful_diagnostic(
                now_ms,
                "idle_tantrum",
                "blocked_cooldown",
                Some("tantrum"),
                None,
            )]);
        }

        Some(self.actions_for_tantrum(now_ms, state))
    }

    fn actions_for_tantrum(&self, now_ms: i64, state: &IanState) -> Vec<IanAction> {
        let left = self.movement_boundary.constrain_target(
            state.position.clone(),
            Position {
                x: state.position.x - 52.0,
                y: state.position.y + 18.0,
            },
            &state.behavior_mode,
        );
        let right = self.movement_boundary.constrain_target(
            left.clone(),
            Position {
                x: state.position.x + 58.0,
                y: state.position.y + 10.0,
            },
            &state.behavior_mode,
        );

        vec![
            self.playful_diagnostic(
                now_ms,
                "idle_tantrum",
                "triggered",
                Some("tantrum"),
                Some("tantrum_roll"),
            ),
            IanAction::PlayfulStateSet {
                state: PlayfulState::WarmingUp,
                until_ms: Some(now_ms + 2_400),
            },
            IanAction::SpeechShow {
                text: "我想打个小滚。".to_string(),
                mood: Some("calm".to_string()),
                duration_ms: Some(1800),
            },
            IanAction::EffectPlay {
                name: "blush_puff".to_string(),
                intensity: "medium".to_string(),
                duration_ms: 900,
            },
            IanAction::AnimationPlay {
                name: "run".to_string(),
                looped: true,
            },
            IanAction::MovementMoveTo {
                x: left.x,
                y: left.y,
                speed: MovementSpeed::Fast,
            },
            IanAction::MovementMoveTo {
                x: right.x,
                y: right.y,
                speed: MovementSpeed::Fast,
            },
            IanAction::AnimationPlay {
                name: "idle".to_string(),
                looped: true,
            },
            IanAction::PlayfulStateSet {
                state: PlayfulState::CoolingDown,
                until_ms: Some(now_ms + 120_000),
            },
        ]
    }

    fn actions_for_pointer_chase(
        &self,
        now_ms: i64,
        pointer: Position,
        state: &IanState,
    ) -> Vec<IanAction> {
        if is_user_interaction_active(state)
            || is_reduced_motion_enabled(state)
            || matches!(state.behavior_mode, crate::protocol::BehaviorMode::Quiet)
            || matches!(state.playful_energy, crate::protocol::PlayfulEnergy::Off)
            || state
                .quiet_hours
                .is_active_at_minute(minute_of_day_from_epoch_ms(now_ms))
            || state
                .active_app_category
                .as_deref()
                .map(|category| {
                    self.policy
                        .should_reduce_disturbance_for_app_category(category)
                })
                .unwrap_or(false)
        {
            return Vec::new();
        }

        if matches!(
            state.playful_state,
            PlayfulState::WarmingUp
                | PlayfulState::Zooming
                | PlayfulState::Settling
                | PlayfulState::CoolingDown
        ) && state
            .playful_state_until_ms
            .map(|until| now_ms < until)
            .unwrap_or(true)
        {
            return vec![self.playful_diagnostic(
                now_ms,
                "pointer_chase",
                "blocked_cooldown",
                Some("pointer_chase"),
                None,
            )];
        }

        let dx = pointer.x - state.position.x;
        let dy = pointer.y - state.position.y;
        let distance = (dx.powi(2) + dy.powi(2)).sqrt();
        if !(90.0..=720.0).contains(&distance) {
            return vec![self.playful_diagnostic(
                now_ms,
                "pointer_chase",
                if distance < 90.0 {
                    "blocked_too_close"
                } else {
                    "blocked_too_far"
                },
                Some("pointer_chase"),
                None,
            )];
        }

        let step = match state.behavior_mode {
            crate::protocol::BehaviorMode::Lively => 150.0,
            crate::protocol::BehaviorMode::Normal => 110.0,
            crate::protocol::BehaviorMode::Quiet => 0.0,
        };
        let ratio = (step / distance).min(0.72);
        let first_target = self.movement_boundary.constrain_target(
            state.position.clone(),
            Position {
                x: state.position.x + dx * ratio * 0.45,
                y: state.position.y + dy * ratio * 0.45,
            },
            &state.behavior_mode,
        );
        let chase_target = self.movement_boundary.constrain_target(
            first_target.clone(),
            Position {
                x: state.position.x + dx * ratio,
                y: state.position.y + dy * ratio,
            },
            &state.behavior_mode,
        );
        let settle_target = self.movement_boundary.constrain_target(
            chase_target.clone(),
            Position {
                x: state.position.x + dx * ratio * 0.92,
                y: state.position.y + dy * ratio * 0.92,
            },
            &state.behavior_mode,
        );

        vec![
            self.playful_diagnostic(
                now_ms,
                "pointer_chase",
                "triggered",
                Some("pointer_chase"),
                Some("cursor_dash"),
            ),
            IanAction::PlayfulStateSet {
                state: PlayfulState::WarmingUp,
                until_ms: Some(now_ms + 1_800),
            },
            IanAction::EffectPlay {
                name: "speed_lines".to_string(),
                intensity: self
                    .policy
                    .playful_effect_intensity(&state.playful_energy)
                    .to_string(),
                duration_ms: 700,
            },
            IanAction::AnimationPlay {
                name: "run".to_string(),
                looped: true,
            },
            IanAction::MovementMoveTo {
                x: first_target.x,
                y: first_target.y,
                speed: MovementSpeed::Fast,
            },
            IanAction::MovementMoveTo {
                x: chase_target.x,
                y: chase_target.y,
                speed: MovementSpeed::Fast,
            },
            IanAction::MovementMoveTo {
                x: settle_target.x,
                y: settle_target.y,
                speed: MovementSpeed::Fast,
            },
            IanAction::AnimationPlay {
                name: "idle".to_string(),
                looped: true,
            },
            IanAction::PlayfulStateSet {
                state: PlayfulState::CoolingDown,
                until_ms: Some(now_ms + 10_000),
            },
        ]
    }

    fn actions_for_playful_tick(&self, now_ms: i64, state: &IanState) -> Option<Vec<IanAction>> {
        if state
            .playful_snoozed_until_ms
            .map(|until| now_ms < until)
            .unwrap_or(false)
        {
            return Some(vec![self.playful_diagnostic(
                now_ms,
                "idle_surprise",
                "blocked_snooze",
                Some("zoomies"),
                None,
            )]);
        }

        if matches!(
            state.playful_state,
            PlayfulState::Zooming | PlayfulState::CoolingDown
        ) && state
            .playful_state_until_ms
            .map(|until| now_ms < until)
            .unwrap_or(true)
        {
            return Some(vec![self.playful_diagnostic(
                now_ms,
                "idle_surprise",
                "blocked_cooldown",
                Some("zoomies"),
                None,
            )]);
        }

        let interval = self
            .policy
            .playful_trigger_interval_secs(&state.playful_energy)?;
        let second = now_ms.div_euclid(1000);
        if second < interval || second.rem_euclid(interval) >= 15 {
            return None;
        }

        Some(self.actions_for_zoomies(now_ms, state, "idle_surprise"))
    }

    fn actions_for_zoomies(&self, now_ms: i64, state: &IanState, reason: &str) -> Vec<IanAction> {
        let offsets = [
            (140.0, -80.0),
            (-120.0, 90.0),
            (180.0, 70.0),
            (-90.0, -70.0),
            (60.0, 35.0),
        ];
        let mut actions = vec![
            self.playful_diagnostic(
                now_ms,
                reason,
                "triggered",
                Some("zoomies"),
                Some("zoomies_path"),
            ),
            IanAction::PlayfulStateSet {
                state: PlayfulState::Zooming,
                until_ms: Some(now_ms + 3_200),
            },
            IanAction::BehaviorZoomies {
                duration_ms: 3_200,
                reason: reason.to_string(),
            },
            IanAction::AnimationPlay {
                name: "zoomies".to_string(),
                looped: true,
            },
            IanAction::EffectPlay {
                name: "speed_lines".to_string(),
                intensity: self
                    .policy
                    .playful_effect_intensity(&state.playful_energy)
                    .to_string(),
                duration_ms: 900,
            },
        ];

        let mut random = ControlledRandom::seeded(now_ms as u64 ^ 0x1A4E);
        let start = random.next_index(offsets.len());

        for index in 0..offsets.len() {
            let (x, y) = offsets[(start + index) % offsets.len()];
            let target = self.movement_boundary.constrain_target(
                state.position.clone(),
                Position {
                    x: state.position.x + x,
                    y: state.position.y + y,
                },
                &state.behavior_mode,
            );
            actions.push(IanAction::MovementMoveTo {
                x: target.x,
                y: target.y,
                speed: MovementSpeed::Fast,
            });
        }

        actions.push(IanAction::MovementMoveTo {
            x: state.home_anchor.x,
            y: state.home_anchor.y,
            speed: MovementSpeed::Fast,
        });
        actions.push(IanAction::PlayfulStateSet {
            state: PlayfulState::Settling,
            until_ms: Some(now_ms + 4_200),
        });
        actions.push(IanAction::EffectPlay {
            name: "blush_puff".to_string(),
            intensity: "low".to_string(),
            duration_ms: 800,
        });
        actions.push(IanAction::AnimationPlay {
            name: "idle".to_string(),
            looped: true,
        });
        actions.push(IanAction::PlayfulStateSet {
            state: PlayfulState::CoolingDown,
            until_ms: Some(now_ms + 60_000),
        });
        actions
    }

    fn actions_for_affection_phrase(&self, text: String) -> Vec<IanAction> {
        vec![
            IanAction::BubbleOpen,
            IanAction::SpeechShow {
                text,
                mood: Some("calm".to_string()),
                duration_ms: Some(2200),
            },
            IanAction::AnimationPlay {
                name: "happy".to_string(),
                looped: false,
            },
            IanAction::EffectPlay {
                name: "heart_pop".to_string(),
                intensity: "low".to_string(),
                duration_ms: 900,
            },
        ]
    }

    fn recent_wake_phrase(&self) -> Option<&'static str> {
        self.momentary_life
            .lock()
            .ok()
            .and_then(|mut life| life.consume_recent_wake_phrase())
    }

    fn record_momentary_event(&self, event: &IanEvent) {
        if let Ok(mut life) = self.momentary_life.lock() {
            life.record_event(event);
        }
    }

    fn record_momentary_actions(&self, actions: &[IanAction]) {
        if let Ok(mut life) = self.momentary_life.lock() {
            life.record_actions(actions);
        }
    }

    fn playful_diagnostic(
        &self,
        now_ms: i64,
        reason: &str,
        result: &str,
        cooldown_key: Option<&str>,
        chosen_reaction_key: Option<&str>,
    ) -> IanAction {
        IanAction::PlayfulDiagnostic {
            timestamp_ms: now_ms,
            reason: reason.to_string(),
            result: result.to_string(),
            cooldown_key: cooldown_key.map(str::to_string),
            chosen_reaction_key: chosen_reaction_key.map(str::to_string),
        }
    }

    fn moment_decision(
        &self,
        kind: IanMomentKind,
        now_ms: i64,
        state: &IanState,
    ) -> super::moment_orchestrator::MomentDecision {
        self.moments
            .lock()
            .map(|mut moments| moments.decide(kind, now_ms, state))
            .unwrap_or(super::moment_orchestrator::MomentDecision {
                kind,
                result: "blocked_context",
            })
    }
}

fn is_user_interaction_active(state: &IanState) -> bool {
    state.is_dragging || state.is_bubble_input_active || state.current_animation == "run"
}

fn is_active_settling_state(now_ms: i64, state: &IanState) -> bool {
    matches!(state.playful_state, PlayfulState::Settling)
        && state
            .playful_state_until_ms
            .map(|until| now_ms < until)
            .unwrap_or(false)
}

fn is_position_visible(state: &IanState) -> bool {
    let Some(bounds) = &state.screen_bounds else {
        return false;
    };

    state.position.x >= bounds.x
        && state.position.y >= bounds.y
        && state.position.x <= bounds.x + bounds.width - 80.0
        && state.position.y <= bounds.y + bounds.height - 80.0
}

fn is_reduced_motion_enabled(state: &IanState) -> bool {
    state.movement_intensity == "reduced"
}

fn find_ian_target(state: &IanState) -> Position {
    if let Some(bounds) = &state.screen_bounds {
        return Position {
            x: bounds.x + (bounds.width - 220.0).max(24.0),
            y: bounds.y + (bounds.height - 260.0).max(24.0),
        };
    }

    Position { x: 48.0, y: 48.0 }
}

#[cfg(test)]
mod tests {
    use crate::protocol::{IanAction, IanEvent, IanState, PlayfulEnergy, PlayfulState, Position};

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
    fn app_started_emits_short_life_intro_without_moving_position() {
        let engine = BehaviorEngine::default();
        let state = IanState::default();

        let actions = engine.decide(&IanEvent::AppStarted, &state);

        assert!(plays_animation(&actions, "happy"));
        assert!(actions.iter().any(|action| matches!(
            action,
            IanAction::EffectPlay { name, intensity, .. } if name == "sparkle_pop" && intensity == "low"
        )));
        assert!(actions
            .iter()
            .any(|action| matches!(action, IanAction::BubbleOpen)));
        assert!(actions.iter().any(|action| matches!(
            action,
            IanAction::SpeechShow { text, .. } if text == "醒啦。"
        )));
        assert!(!actions
            .iter()
            .any(|action| matches!(action, IanAction::MovementMoveTo { .. })));
    }

    #[test]
    fn drag_start_and_end_have_distinct_touch_reactions() {
        let engine = BehaviorEngine::default();
        let state = IanState::default();

        let start = engine.decide(&IanEvent::MouseDragStart { x: 1.0, y: 1.0 }, &state);
        let end = engine.decide(&IanEvent::MouseDragEnd { x: 12.0, y: 16.0 }, &state);

        assert!(plays_animation(&start, "happy"));
        assert!(start.iter().any(|action| matches!(
            action,
            IanAction::EffectPlay { name, .. } if name == "sparkle_pop"
        )));
        assert!(start.iter().any(|action| matches!(
            action,
            IanAction::SpeechShow { text, .. } if text == "抱起来啦。"
        )));
        assert!(end.iter().any(|action| matches!(
            action,
            IanAction::SpeechShow { text, .. } if text == "放这里。"
        )));
        assert!(end.iter().any(|action| matches!(
            action,
            IanAction::EffectPlay { name, .. } if name == "blush_puff"
        )));
        assert!(end.iter().any(|action| matches!(
            action,
            IanAction::MovementMoveTo { x, y, .. } if *x == 12.0 && *y == 16.0
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

        let idle_actions = engine.decide(&IanEvent::TimeTick { now_ms: 135_000 }, &state);
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
    fn movement_personality_controls_autonomous_movement_frequency() {
        let engine = BehaviorEngine::default();
        let mut quiet = IanState::default();
        quiet.behavior_mode = crate::protocol::BehaviorMode::Quiet;
        let mut normal = IanState::default();
        normal.behavior_mode = crate::protocol::BehaviorMode::Normal;
        let mut lively = IanState::default();
        lively.behavior_mode = crate::protocol::BehaviorMode::Lively;

        let quiet_45s = engine.decide(&IanEvent::TimeTick { now_ms: 45_000 }, &quiet);
        let normal_45s = engine.decide(&IanEvent::TimeTick { now_ms: 45_000 }, &normal);
        let normal_135s = engine.decide(&IanEvent::TimeTick { now_ms: 135_000 }, &normal);
        let lively_75s = engine.decide(&IanEvent::TimeTick { now_ms: 75_000 }, &lively);

        assert_no_autonomous_movement_or_sleep(&quiet_45s);
        assert!(!has_movement(&normal_45s));
        assert!(plays_animation(&normal_45s, "rest"));
        assert!(has_movement(&normal_135s));
        assert!(has_movement(&lively_75s));
    }

    #[test]
    fn autonomous_roam_uses_visible_short_path_and_walk_animation() {
        let engine = BehaviorEngine::default();
        let mut state = IanState::default();
        state.position.x = 160.0;
        state.position.y = 140.0;

        let actions = engine.decide(&IanEvent::TimeTick { now_ms: 135_000 }, &state);

        assert!(plays_animation(&actions, "walk"));
        let movement_count = actions
            .iter()
            .filter(|action| matches!(action, IanAction::MovementMoveTo { .. }))
            .count();
        assert!(movement_count >= 2);
        assert!(matches!(
            actions.iter().rev().find(|action| matches!(action, IanAction::AnimationPlay { .. })),
            Some(IanAction::AnimationPlay { name, looped }) if name == "idle" && *looped
        ));
    }

    #[test]
    fn idle_tick_can_start_tiny_perimeter_patrol_from_top_left() {
        let engine = BehaviorEngine::default();
        let mut state = IanState::default();
        state.position.x = 640.0;
        state.position.y = 420.0;
        state.screen_bounds = Some(crate::protocol::ScreenBounds {
            x: 0.0,
            y: 0.0,
            width: 1440.0,
            height: 900.0,
        });

        let actions = engine.decide(&IanEvent::TimeTick { now_ms: 61_000 }, &state);

        assert!(actions.iter().any(|action| matches!(
            action,
            IanAction::AppearanceScaleTo { scale, .. } if (*scale - 0.45).abs() < f64::EPSILON
        )));
        assert!(actions.iter().any(|action| matches!(
            action,
            IanAction::MovementMoveTo { x, y, speed }
                if *x <= 48.0 && *y <= 48.0 && matches!(speed, crate::protocol::MovementSpeed::Slow)
        )));
    }

    #[test]
    fn idle_perimeter_patrol_emits_a_full_edge_loop_in_one_sequence() {
        let engine = BehaviorEngine::default();
        let mut state = IanState::default();
        state.position.x = 64.0;
        state.position.y = 720.0;
        state.screen_bounds = Some(crate::protocol::ScreenBounds {
            x: 0.0,
            y: 0.0,
            width: 1440.0,
            height: 900.0,
        });

        let actions = engine.decide(&IanEvent::TimeTick { now_ms: 61_000 }, &state);
        let targets: Vec<(f64, f64)> = actions
            .iter()
            .filter_map(|action| match action {
                IanAction::MovementMoveTo { x, y, .. } => Some((*x, *y)),
                _ => None,
            })
            .collect();

        assert!(
            targets.len() >= 5,
            "perimeter patrol should include entry point and all four edges, got {targets:?}"
        );
        assert!(targets[0].0 <= 48.0 && targets[0].1 <= 48.0);
        assert!(targets[1].0 > 1000.0 && targets[1].1 <= 48.0);
        assert!(targets[2].0 > 1000.0 && targets[2].1 > 600.0);
        assert!(targets[3].0 <= 48.0 && targets[3].1 > 600.0);
        assert!(targets[4].0 <= 48.0 && targets[4].1 <= 48.0);
    }

    #[test]
    fn idle_perimeter_patrol_compensates_for_desktop_window_visual_offset() {
        let engine = BehaviorEngine::default();
        let mut state = IanState::default();
        state.position.x = 64.0;
        state.position.y = 720.0;
        state.screen_bounds = Some(crate::protocol::ScreenBounds {
            x: 0.0,
            y: 0.0,
            width: 1440.0,
            height: 900.0,
        });

        let actions = engine.decide(&IanEvent::TimeTick { now_ms: 61_000 }, &state);
        let first_target = actions.iter().find_map(|action| match action {
            IanAction::MovementMoveTo { x, y, .. } => Some((*x, *y)),
            _ => None,
        });

        assert!(matches!(first_target, Some((x, y)) if x < 0.0 && y < 0.0));
    }

    #[test]
    fn idle_perimeter_patrol_does_not_restart_while_patrol_is_active() {
        let engine = BehaviorEngine::default();
        let mut state = IanState::default();
        state.position.x = 64.0;
        state.position.y = 720.0;
        state.playful_state = PlayfulState::Settling;
        state.playful_state_until_ms = Some(180_000);
        state.screen_bounds = Some(crate::protocol::ScreenBounds {
            x: 0.0,
            y: 0.0,
            width: 1440.0,
            height: 900.0,
        });

        let actions = engine.decide(&IanEvent::TimeTick { now_ms: 75_000 }, &state);

        assert!(!actions
            .iter()
            .any(|action| matches!(action, IanAction::MovementMoveTo { .. })));
        assert!(!actions
            .iter()
            .any(|action| matches!(action, IanAction::AppearanceScaleTo { .. })));
    }

    #[test]
    fn idle_perimeter_patrol_blocks_autonomous_animation_ticks_while_active() {
        let engine = BehaviorEngine::default();
        let mut state = IanState::default();
        state.current_animation = "walk".to_string();
        state.playful_state = PlayfulState::Settling;
        state.playful_state_until_ms = Some(180_000);
        state.screen_bounds = Some(crate::protocol::ScreenBounds {
            x: 0.0,
            y: 0.0,
            width: 1440.0,
            height: 900.0,
        });

        let actions = engine.decide(&IanEvent::TimeTick { now_ms: 90_000 }, &state);

        assert!(
            !actions
                .iter()
                .any(|action| matches!(action, IanAction::AnimationPlay { .. })),
            "active perimeter patrol should not be interrupted by scheduler animation actions: {actions:?}"
        );
    }

    #[test]
    fn pointer_chase_does_not_interrupt_active_perimeter_patrol() {
        let engine = BehaviorEngine::default();
        let mut state = IanState::default();
        state.position = Position { x: 300.0, y: 300.0 };
        state.playful_state = PlayfulState::Settling;
        state.playful_state_until_ms = Some(180_000);

        let actions = engine.decide(
            &IanEvent::MouseChaseCandidate {
                x: 420.0,
                y: 300.0,
                now_ms: 90_000,
            },
            &state,
        );

        assert!(
            !actions
                .iter()
                .any(|action| matches!(action, IanAction::MovementMoveTo { .. })),
            "pointer chase should not inject movement while perimeter patrol is active: {actions:?}"
        );
        assert!(
            !actions
                .iter()
                .any(|action| matches!(action, IanAction::AnimationPlay { name, .. } if name == "run")),
            "pointer chase should not switch animation to run while perimeter patrol is active: {actions:?}"
        );
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
    fn affectionate_reactions_offer_multiple_light_visual_effects() {
        let engine = BehaviorEngine::default();
        let state = IanState::default();

        let first = engine.decide(&IanEvent::MouseClick { x: 1.0, y: 1.0 }, &state);
        let second = engine.decide(&IanEvent::MouseClick { x: 1.0, y: 1.0 }, &state);
        let third = engine.decide(&IanEvent::MouseClick { x: 1.0, y: 1.0 }, &state);
        let fourth = engine.decide(&IanEvent::MouseClick { x: 1.0, y: 1.0 }, &state);

        let effect_names = first
            .iter()
            .chain(second.iter())
            .chain(third.iter())
            .chain(fourth.iter())
            .filter_map(|action| {
                if let IanAction::EffectPlay { name, .. } = action {
                    Some(name.as_str())
                } else {
                    None
                }
            })
            .collect::<std::collections::BTreeSet<_>>();

        assert!(effect_names.contains("heart_pop"));
        assert!(effect_names.contains("sparkle_pop"));
        assert!(effect_names.contains("blush_puff"));
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
    fn time_tick_returns_predictable_idle_rest_sleep_visual_states() {
        let engine = BehaviorEngine::default();
        let state = IanState::default();

        let idle_actions = engine.decide(&IanEvent::TimeTick { now_ms: 1_000 }, &state);
        let rest_actions = engine.decide(&IanEvent::TimeTick { now_ms: 45_000 }, &state);
        let sleep_actions = engine.decide(&IanEvent::TimeTick { now_ms: 90_000 }, &state);

        assert!(idle_actions.iter().any(|action| matches!(
            action,
            IanAction::AnimationPlay { name, looped } if name == "idle" && *looped
        )));
        assert!(rest_actions.iter().any(|action| matches!(
            action,
            IanAction::AnimationPlay { name, looped } if name == "rest" && *looped
        )));
        assert!(sleep_actions.iter().any(|action| matches!(
            action,
            IanAction::AnimationPlay { name, looped } if name == "sleep" && *looped
        )));
    }

    #[test]
    fn playful_energy_off_and_safety_gates_block_spontaneous_zoomies() {
        let engine = BehaviorEngine::default();
        let mut off = IanState::default();
        off.playful_energy = PlayfulEnergy::Off;
        let mut quiet = IanState::default();
        quiet.behavior_mode = crate::protocol::BehaviorMode::Quiet;
        let mut typing = IanState::default();
        typing.is_bubble_input_active = true;

        assert!(!has_zoomies(
            &engine.decide(&IanEvent::TimeTick { now_ms: 180_000 }, &off)
        ));
        assert!(!has_zoomies(
            &engine.decide(&IanEvent::TimeTick { now_ms: 180_000 }, &quiet)
        ));
        assert!(!has_zoomies(
            &engine.decide(&IanEvent::TimeTick { now_ms: 180_000 }, &typing)
        ));
    }

    #[test]
    fn high_playful_energy_can_emit_bounded_zoomies_with_diagnostics() {
        let engine = BehaviorEngine::default();
        let mut state = IanState::default();
        state.playful_energy = PlayfulEnergy::High;
        state.position.x = 100.0;
        state.position.y = 100.0;
        state.home_anchor = state.position.clone();

        let actions = engine.decide(&IanEvent::TimeTick { now_ms: 180_000 }, &state);

        assert!(has_zoomies(&actions));
        assert!(plays_animation(&actions, "zoomies"));
        assert!(actions.iter().any(|action| matches!(
            action,
            IanAction::EffectPlay { name, intensity, .. } if name == "speed_lines" && intensity == "high"
        )));
        let movement_count = actions
            .iter()
            .filter(|action| matches!(action, IanAction::MovementMoveTo { .. }))
            .count();
        assert!((4..=8).contains(&movement_count));
        assert!(actions.iter().any(|action| matches!(
            action,
            IanAction::PlayfulDiagnostic { reason, result, .. }
                if reason == "idle_surprise" && result == "triggered"
        )));
    }

    #[test]
    fn lively_high_energy_can_emit_cooldown_protected_tantrum() {
        let engine = BehaviorEngine::default();
        let mut state = IanState::default();
        state.behavior_mode = crate::protocol::BehaviorMode::Lively;
        state.playful_energy = PlayfulEnergy::High;
        state.position.x = 180.0;
        state.position.y = 180.0;

        let actions = engine.decide(&IanEvent::TimeTick { now_ms: 300_000 }, &state);

        assert!(actions.iter().any(|action| matches!(
            action,
            IanAction::PlayfulDiagnostic { reason, result, chosen_reaction_key, .. }
                if reason == "idle_tantrum"
                    && result == "triggered"
                    && chosen_reaction_key.as_deref() == Some("tantrum_roll")
        )));
        assert!(actions.iter().any(|action| matches!(
            action,
            IanAction::SpeechShow { text, .. } if text.contains("小滚") && !text.contains("不理")
        )));
        assert!(actions.iter().any(|action| matches!(
            action,
            IanAction::PlayfulStateSet { state, until_ms: Some(until_ms) }
                if matches!(state, PlayfulState::CoolingDown) && *until_ms > 300_000
        )));
    }

    #[test]
    fn tantrum_respects_interaction_and_cooldown_gates() {
        let engine = BehaviorEngine::default();
        let mut dragging = IanState::default();
        dragging.behavior_mode = crate::protocol::BehaviorMode::Lively;
        dragging.playful_energy = PlayfulEnergy::High;
        dragging.is_dragging = true;
        let mut cooling = IanState::default();
        cooling.behavior_mode = crate::protocol::BehaviorMode::Lively;
        cooling.playful_energy = PlayfulEnergy::High;
        cooling.playful_state = PlayfulState::CoolingDown;
        cooling.playful_state_until_ms = Some(360_000);

        let dragging_actions = engine.decide(&IanEvent::TimeTick { now_ms: 300_000 }, &dragging);
        let cooling_actions = engine.decide(&IanEvent::TimeTick { now_ms: 300_000 }, &cooling);

        assert!(!has_tantrum(&dragging_actions));
        assert!(!has_tantrum(&cooling_actions));
    }

    #[test]
    fn pointer_chase_moves_toward_nearby_cursor_without_jumping_to_it() {
        let engine = BehaviorEngine::default();
        let mut state = IanState::default();
        state.behavior_mode = crate::protocol::BehaviorMode::Lively;
        state.playful_energy = PlayfulEnergy::High;
        state.position = crate::protocol::Position { x: 300.0, y: 300.0 };

        let actions = engine.decide(
            &IanEvent::MouseChaseCandidate {
                x: 520.0,
                y: 430.0,
                now_ms: 300_000,
            },
            &state,
        );

        assert!(actions.iter().any(|action| matches!(
            action,
            IanAction::PlayfulDiagnostic { reason, result, chosen_reaction_key, .. }
                if reason == "pointer_chase"
                    && result == "triggered"
                    && chosen_reaction_key.as_deref() == Some("cursor_dash")
        )));
        assert!(plays_animation(&actions, "run"));
        let movement_targets = actions
            .iter()
            .filter_map(|action| {
                if let IanAction::MovementMoveTo { x, y, speed } = action {
                    Some((*x, *y, speed))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        assert!(movement_targets.len() >= 2);
        assert!(movement_targets.iter().all(|(x, y, speed)| {
            *x > state.position.x
                && *x < 520.0
                && *y > state.position.y
                && *y < 430.0
                && matches!(speed, crate::protocol::MovementSpeed::Fast)
        }));
        let first_target = movement_targets[0];
        let last_target = movement_targets[movement_targets.len() - 1];
        assert!(
            (first_target.0 - last_target.0).abs() > f64::EPSILON
                || (first_target.1 - last_target.1).abs() > f64::EPSILON
        );
        assert!(actions.iter().any(|action| matches!(
            action,
            IanAction::PlayfulStateSet { state, until_ms: Some(until_ms) }
                if matches!(state, PlayfulState::CoolingDown) && *until_ms == 310_000
        )));
    }

    #[test]
    fn pointer_chase_cooldown_is_short_enough_for_desktop_feel_testing() {
        let engine = BehaviorEngine::default();
        let mut state = IanState::default();
        state.behavior_mode = crate::protocol::BehaviorMode::Lively;
        state.playful_energy = PlayfulEnergy::High;
        state.position = crate::protocol::Position { x: 300.0, y: 300.0 };

        let triggered = engine.decide(
            &IanEvent::MouseChaseCandidate {
                x: 520.0,
                y: 430.0,
                now_ms: 300_000,
            },
            &state,
        );
        let mut cooling = state.clone();
        cooling.playful_state = PlayfulState::CoolingDown;
        cooling.playful_state_until_ms = Some(310_000);
        let blocked = engine.decide(
            &IanEvent::MouseChaseCandidate {
                x: 520.0,
                y: 430.0,
                now_ms: 305_000,
            },
            &cooling,
        );
        let mut recovered = cooling;
        recovered.playful_state_until_ms = Some(310_000);
        let retry = engine.decide(
            &IanEvent::MouseChaseCandidate {
                x: 520.0,
                y: 430.0,
                now_ms: 311_000,
            },
            &recovered,
        );

        assert!(has_pointer_chase(&triggered));
        assert!(blocked.iter().any(|action| matches!(
            action,
            IanAction::PlayfulDiagnostic { reason, result, cooldown_key, .. }
                if reason == "pointer_chase"
                    && result == "blocked_cooldown"
                    && cooldown_key.as_deref() == Some("pointer_chase")
        )));
        assert!(has_pointer_chase(&retry));
    }

    #[test]
    fn pointer_chase_blocks_when_distance_or_user_context_is_wrong() {
        let engine = BehaviorEngine::default();
        let mut state = IanState::default();
        state.behavior_mode = crate::protocol::BehaviorMode::Lively;
        state.playful_energy = PlayfulEnergy::High;
        state.position = crate::protocol::Position { x: 300.0, y: 300.0 };

        let too_close = engine.decide(
            &IanEvent::MouseChaseCandidate {
                x: 320.0,
                y: 318.0,
                now_ms: 300_000,
            },
            &state,
        );
        let too_far = engine.decide(
            &IanEvent::MouseChaseCandidate {
                x: 1_500.0,
                y: 1_200.0,
                now_ms: 300_000,
            },
            &state,
        );
        let mut typing = state.clone();
        typing.is_bubble_input_active = true;
        let typing_actions = engine.decide(
            &IanEvent::MouseChaseCandidate {
                x: 520.0,
                y: 430.0,
                now_ms: 300_000,
            },
            &typing,
        );
        let mut cooling = state;
        cooling.playful_state = PlayfulState::CoolingDown;
        cooling.playful_state_until_ms = Some(360_000);
        let cooling_actions = engine.decide(
            &IanEvent::MouseChaseCandidate {
                x: 520.0,
                y: 430.0,
                now_ms: 300_000,
            },
            &cooling,
        );

        assert!(!has_pointer_chase(&too_close));
        assert!(!has_pointer_chase(&too_far));
        assert!(typing_actions.is_empty());
        assert!(!has_pointer_chase(&cooling_actions));
    }

    #[test]
    fn playful_state_and_cooldown_prevent_stacked_zoomies_without_blocking_clicks() {
        let engine = BehaviorEngine::default();
        let mut cooling = IanState::default();
        cooling.playful_energy = PlayfulEnergy::High;
        cooling.playful_state = PlayfulState::CoolingDown;
        cooling.playful_state_until_ms = Some(240_000);

        let tick = engine.decide(&IanEvent::TimeTick { now_ms: 190_000 }, &cooling);
        let click = engine.decide(&IanEvent::MouseClick { x: 1.0, y: 1.0 }, &cooling);

        assert!(!has_zoomies(&tick));
        assert!(tick.iter().any(|action| matches!(
            action,
            IanAction::PlayfulDiagnostic { result, cooldown_key, .. }
                if result == "blocked_cooldown" && cooldown_key.as_deref() == Some("zoomies")
        )));
        assert!(click
            .iter()
            .any(|action| matches!(action, IanAction::SpeechShow { .. })));
    }

    #[test]
    fn mouse_near_can_emit_a_short_reaction_chain_without_interrupting_interaction() {
        let engine = BehaviorEngine::default();
        let idle = IanState::default();
        let mut typing = IanState::default();
        typing.is_bubble_input_active = true;

        let chain = engine.decide(
            &IanEvent::MouseNear {
                x: 32.0,
                y: 48.0,
                now_ms: 1_000,
            },
            &idle,
        );
        let typing_actions = engine.decide(
            &IanEvent::MouseNear {
                x: 32.0,
                y: 48.0,
                now_ms: 7_000,
            },
            &typing,
        );

        assert!(plays_animation(&chain, "happy"));
        assert!(chain.iter().any(|action| matches!(
            action,
            IanAction::EffectPlay { name, .. } if name == "sparkle_pop"
        )));
        assert!(chain.iter().any(|action| matches!(
            action,
            IanAction::SpeechShow { text, .. } if text == "看到你啦。"
        )));
        assert!(typing_actions.is_empty());
    }

    #[test]
    fn playful_diagnostics_are_low_sensitive_reason_fields_only() {
        let engine = BehaviorEngine::default();
        let mut state = IanState::default();
        state.playful_energy = PlayfulEnergy::High;

        let actions = engine.decide(&IanEvent::TimeTick { now_ms: 180_000 }, &state);
        let diagnostic = actions.iter().find_map(|action| {
            if let IanAction::PlayfulDiagnostic {
                reason,
                result,
                cooldown_key,
                chosen_reaction_key,
                ..
            } = action
            {
                Some(format!(
                    "{reason}:{result}:{:?}:{:?}",
                    cooldown_key, chosen_reaction_key
                ))
            } else {
                None
            }
        });

        let diagnostic = diagnostic.expect("playful diagnostic");
        assert!(!diagnostic.contains("window"));
        assert!(!diagnostic.contains("http"));
        assert!(!diagnostic.contains('/'));
    }

    #[test]
    fn cute_reaction_pack_has_varied_local_phrases_without_sensitive_content() {
        let engine = BehaviorEngine::default();
        let state = IanState::default();
        let mut phrases = std::collections::BTreeSet::new();

        for _ in 0..6 {
            for action in engine.decide(&IanEvent::MouseClick { x: 1.0, y: 1.0 }, &state) {
                if let IanAction::SpeechShow { text, .. } = action {
                    phrases.insert(text);
                }
            }
        }

        assert!(phrases.len() >= 3);
        assert!(phrases
            .iter()
            .all(|phrase| !phrase.contains("AI") && !phrase.contains("助手")));
    }

    #[test]
    fn idle_tick_can_emit_low_disturbance_micro_motion() {
        let engine = BehaviorEngine::default();
        let state = IanState::default();

        let actions = engine.decide(&IanEvent::TimeTick { now_ms: 15_000 }, &state);

        assert!(actions.iter().any(|action| matches!(
            action,
            IanAction::EffectPlay { name, intensity, .. }
                if name == "tail_wag" && intensity == "low"
        )));
        assert!(!actions
            .iter()
            .any(|action| matches!(action, IanAction::MovementMoveTo { .. })));
        assert!(!actions
            .iter()
            .any(|action| matches!(action, IanAction::SpeechShow { .. })));
    }

    #[test]
    fn idle_micro_motion_respects_quiet_and_interaction_gates() {
        let engine = BehaviorEngine::default();
        let mut quiet = IanState::default();
        quiet.behavior_mode = crate::protocol::BehaviorMode::Quiet;
        let mut typing = IanState::default();
        typing.is_bubble_input_active = true;
        let mut dragging = IanState::default();
        dragging.is_dragging = true;

        let quiet_actions = engine.decide(&IanEvent::TimeTick { now_ms: 15_000 }, &quiet);
        let typing_actions = engine.decide(&IanEvent::TimeTick { now_ms: 15_000 }, &typing);
        let dragging_actions = engine.decide(&IanEvent::TimeTick { now_ms: 15_000 }, &dragging);

        assert!(!has_effect(&quiet_actions, "tail_wag"));
        assert!(!has_effect(&typing_actions, "tail_wag"));
        assert!(!has_effect(&dragging_actions, "tail_wag"));
    }

    #[test]
    fn drag_end_and_run_around_include_settle_steps() {
        let engine = BehaviorEngine::default();
        let mut state = IanState::default();
        state.position.x = 100.0;
        state.position.y = 100.0;
        state.home_anchor = state.position.clone();

        let drag_end = engine.decide(&IanEvent::MouseDragEnd { x: 120.0, y: 132.0 }, &state);
        let run = engine.decide(&IanEvent::MouseDoubleClick { x: 100.0, y: 100.0 }, &state);

        assert!(matches!(
            drag_end.as_slice(),
            [
                IanAction::MovementMoveTo { .. },
                IanAction::EffectPlay { .. },
                IanAction::AnimationPlay { .. },
                IanAction::SpeechShow { .. }
            ]
        ));
        assert!(run.iter().any(|action| matches!(
            action,
            IanAction::PlayfulStateSet { state, .. } if matches!(state, PlayfulState::Settling)
        )));
        assert!(has_effect(&run, "blush_puff"));
    }

    #[test]
    fn recent_wake_state_changes_click_phrase_then_expires() {
        let engine = BehaviorEngine::default();
        let state = IanState::default();

        let _ = engine.decide(&IanEvent::AppStarted, &state);
        let fresh_click = engine.decide(&IanEvent::MouseClick { x: 1.0, y: 1.0 }, &state);
        let _ = engine.decide(&IanEvent::TimeTick { now_ms: 601_000 }, &state);
        let expired_click = engine.decide(&IanEvent::MouseClick { x: 1.0, y: 1.0 }, &state);

        assert!(fresh_click.iter().any(|action| matches!(
            action,
            IanAction::SpeechShow { text, .. } if text == "刚醒。"
        )));
        assert!(!expired_click.iter().any(|action| matches!(
            action,
            IanAction::SpeechShow { text, .. } if text == "刚醒。"
        )));
    }

    #[test]
    fn find_ian_uses_short_visual_beacon_and_wakes_from_sleep() {
        let engine = BehaviorEngine::default();
        let mut state = IanState::default();
        state.current_animation = "sleep".to_string();

        let actions = engine.decide(
            &IanEvent::SystemShortcutTriggered {
                action: "find_ian".to_string(),
                now_ms: 1_000,
            },
            &state,
        );

        assert!(actions.iter().any(|action| matches!(
            action,
            IanAction::EffectPlay { name, duration_ms, .. } if name == "find_beacon" && *duration_ms <= 2_000
        )));
        assert!(plays_animation(&actions, "happy"));
    }

    #[test]
    fn find_ian_entrance_moment_moves_only_before_cooldown() {
        let engine = BehaviorEngine::default();
        let mut state = IanState::default();
        state.position = Position {
            x: -900.0,
            y: -900.0,
        };
        state.screen_bounds = Some(crate::protocol::ScreenBounds {
            x: 0.0,
            y: 0.0,
            width: 1200.0,
            height: 800.0,
        });

        let first = engine.decide(
            &IanEvent::SystemShortcutTriggered {
                action: "find_ian".to_string(),
                now_ms: 10_000,
            },
            &state,
        );
        let repeated = engine.decide(
            &IanEvent::SystemShortcutTriggered {
                action: "find_ian".to_string(),
                now_ms: 11_000,
            },
            &state,
        );

        assert!(has_movement(&first));
        assert!(first.iter().any(|action| matches!(
            action,
            IanAction::PlayfulDiagnostic { reason, result, .. }
                if reason == "find_ian_entrance" && result == "triggered"
        )));
        assert!(!has_movement(&repeated));
        assert!(repeated.iter().any(|action| matches!(
            action,
            IanAction::PlayfulDiagnostic { reason, result, .. }
                if reason == "find_ian_entrance" && result == "blocked_cooldown"
        )));
    }

    #[test]
    fn pointer_curiosity_moment_is_low_frequency_and_context_gated() {
        let engine = BehaviorEngine::default();
        let state = IanState::default();

        let first = engine.decide(
            &IanEvent::MouseNear {
                x: 32.0,
                y: 48.0,
                now_ms: 20_000,
            },
            &state,
        );
        let repeated = engine.decide(
            &IanEvent::MouseNear {
                x: 32.0,
                y: 48.0,
                now_ms: 21_000,
            },
            &state,
        );
        let mut typing = state.clone();
        typing.is_bubble_input_active = true;
        let typing_actions = engine.decide(
            &IanEvent::MouseNear {
                x: 32.0,
                y: 48.0,
                now_ms: 40_000,
            },
            &typing,
        );

        assert!(first.iter().any(|action| matches!(
            action,
            IanAction::PlayfulDiagnostic { reason, result, .. }
                if reason == "pointer_curiosity" && result == "triggered"
        )));
        assert!(!repeated.iter().any(|action| matches!(
            action,
            IanAction::PlayfulDiagnostic { reason, result, .. }
                if reason == "pointer_curiosity" && result == "triggered"
        )));
        assert!(typing_actions.is_empty());
    }

    #[test]
    fn drag_carry_and_drop_settle_moments_are_short_and_non_blaming() {
        let engine = BehaviorEngine::default();
        let state = IanState::default();

        let carry = engine.decide(&IanEvent::MouseDragStart { x: 1.0, y: 1.0 }, &state);
        let drop = engine.decide(&IanEvent::MouseDragEnd { x: 180.0, y: 120.0 }, &state);

        assert!(carry.iter().any(|action| matches!(
            action,
            IanAction::AppearanceScaleTo { scale, .. } if *scale < 1.0
        )));
        assert!(carry.iter().any(|action| matches!(
            action,
            IanAction::PlayfulDiagnostic { reason, result, .. }
                if reason == "drag_carry" && result == "triggered"
        )));
        assert!(drop.iter().any(|action| matches!(
            action,
            IanAction::PlayfulDiagnostic { reason, result, .. }
                if reason == "drop_settle" && result == "triggered"
        )));
        assert!(!drop.iter().any(|action| matches!(
            action,
            IanAction::SpeechShow { text, .. } if text.contains("怎么") || text.contains("不理")
        )));
    }

    #[test]
    fn rare_idle_surprise_uses_budget_and_respects_reduced_motion() {
        let engine = BehaviorEngine::default();
        let state = IanState::default();

        let first = engine.decide(&IanEvent::TimeTick { now_ms: 3_600_000 }, &state);
        let repeated = engine.decide(&IanEvent::TimeTick { now_ms: 3_600_001 }, &state);
        let mut reduced = state.clone();
        reduced.movement_intensity = "reduced".to_string();
        let reduced_actions = engine.decide(&IanEvent::TimeTick { now_ms: 7_200_000 }, &reduced);

        assert!(first.iter().any(|action| matches!(
            action,
            IanAction::PlayfulDiagnostic { reason, result, .. }
                if reason == "rare_idle_surprise" && result == "triggered"
        )));
        assert!(!repeated.iter().any(|action| matches!(
            action,
            IanAction::PlayfulDiagnostic { reason, result, .. }
                if reason == "rare_idle_surprise" && result == "triggered"
        )));
        assert!(!reduced_actions.iter().any(|action| matches!(
            action,
            IanAction::PlayfulDiagnostic { reason, result, .. }
                if reason == "rare_idle_surprise" && result == "triggered"
        )));
    }

    #[test]
    fn moment_debug_trigger_reaches_core_for_each_known_moment() {
        let engine = BehaviorEngine::default();
        let state = IanState::default();

        for (kind, reason) in [
            ("find_ian_entrance", "find_ian_entrance"),
            ("pointer_curiosity", "pointer_curiosity"),
            ("drag_carry", "drag_carry"),
            ("drop_settle", "drop_settle"),
            ("rare_idle_surprise", "rare_idle_surprise"),
            ("memory_echo", "memory_echo"),
        ] {
            let actions = engine.decide(
                &IanEvent::MomentDebugTrigger {
                    kind: kind.to_string(),
                    now_ms: 42_000,
                },
                &state,
            );

            assert!(
                actions.iter().any(|action| matches!(
                    action,
                    IanAction::PlayfulDiagnostic { reason: actual, result, .. }
                        if actual == reason && result == "diagnostic_triggered"
                )),
                "missing diagnostic for {kind}: {actions:?}"
            );
        }
    }

    #[test]
    fn moment_guardrails_explain_user_control_blocks() {
        let engine = BehaviorEngine::default();
        let mut quiet = IanState::default();
        quiet.playful_energy = PlayfulEnergy::Off;

        let actions = engine.decide(
            &IanEvent::MouseNear {
                x: 32.0,
                y: 48.0,
                now_ms: 20_000,
            },
            &quiet,
        );

        assert!(matches!(
            actions.as_slice(),
            [IanAction::PlayfulDiagnostic { reason, result, .. }]
                if reason == "pointer_curiosity" && result == "blocked_user_control"
        ));
    }

    #[test]
    fn tick_policies_respect_do_not_disturb() {
        let engine = BehaviorEngine::default();
        let mut state = IanState::default();
        state.do_not_disturb = true;

        let actions = engine.decide(&IanEvent::TimeTick { now_ms: 86_400_000 }, &state);

        assert!(!actions.iter().any(|action| matches!(
            action,
            IanAction::SpeechShow { .. } | IanAction::MovementMoveTo { .. }
        )));
        assert!(plays_animation(&actions, "idle"));
    }

    #[test]
    fn reduced_motion_setting_suppresses_autonomous_motion_but_keeps_click_feedback() {
        let engine = BehaviorEngine::default();
        let mut state = IanState::default();
        state.movement_intensity = "reduced".to_string();
        state.playful_energy = PlayfulEnergy::High;

        let roam = engine.decide(&IanEvent::TimeTick { now_ms: 135_000 }, &state);
        let chase = engine.decide(
            &IanEvent::MouseChaseCandidate {
                x: 220.0,
                y: 120.0,
                now_ms: 1_000,
            },
            &state,
        );
        let click = engine.decide(&IanEvent::MouseClick { x: 1.0, y: 1.0 }, &state);

        assert!(!has_movement(&roam));
        assert!(!has_effect(&roam, "tail_wag"));
        assert!(!has_movement(&chase));
        assert!(click.iter().any(|action| matches!(
            action,
            IanAction::SpeechShow { .. } | IanAction::AnimationPlay { .. }
        )));
    }

    #[test]
    fn time_tick_can_emit_daily_greeting_once_after_first_local_day() {
        let engine = BehaviorEngine::default();
        let state = IanState::default();

        let first = engine.decide(&IanEvent::TimeTick { now_ms: 86_400_000 }, &state);
        let repeated = engine.decide(&IanEvent::TimeTick { now_ms: 86_401_000 }, &state);

        assert!(first.iter().any(|action| matches!(
            action,
            IanAction::SpeechShow { text, .. } if text == "早呀。"
        )));
        assert!(!repeated
            .iter()
            .any(|action| matches!(action, IanAction::SpeechShow { .. })));
    }

    #[test]
    fn time_tick_can_emit_absence_return_once_without_blame() {
        let engine = BehaviorEngine::default();
        let mut state = IanState::default();
        state.last_user_interaction_ms = 1_000;

        let first = engine.decide(
            &IanEvent::TimeTick {
                now_ms: 7 * 60 * 60 * 1000,
            },
            &state,
        );
        let repeated = engine.decide(
            &IanEvent::TimeTick {
                now_ms: 7 * 60 * 60 * 1000 + 1_000,
            },
            &state,
        );

        assert!(first.iter().any(|action| matches!(
            action,
            IanAction::SpeechShow { text, .. } if text == "又见到你了。"
        )));
        assert!(!first.iter().any(|action| matches!(
            action,
            IanAction::SpeechShow { text, .. } if text.contains("怎么才") || text.contains("不理")
        )));
        assert!(!repeated
            .iter()
            .any(|action| matches!(action, IanAction::SpeechShow { .. })));
    }

    fn plays_animation(actions: &[IanAction], animation: &str) -> bool {
        actions.iter().any(|action| {
            matches!(
                action,
                IanAction::AnimationPlay { name, .. } if name == animation
            )
        })
    }

    fn has_movement(actions: &[IanAction]) -> bool {
        actions
            .iter()
            .any(|action| matches!(action, IanAction::MovementMoveTo { .. }))
    }

    fn has_zoomies(actions: &[IanAction]) -> bool {
        actions
            .iter()
            .any(|action| matches!(action, IanAction::BehaviorZoomies { .. }))
    }

    fn has_tantrum(actions: &[IanAction]) -> bool {
        actions.iter().any(|action| {
            matches!(
                action,
                IanAction::PlayfulDiagnostic { reason, result, .. }
                    if reason == "idle_tantrum" && result == "triggered"
            )
        })
    }

    fn has_pointer_chase(actions: &[IanAction]) -> bool {
        actions.iter().any(|action| {
            matches!(
                action,
                IanAction::PlayfulDiagnostic { reason, result, .. }
                    if reason == "pointer_chase" && result == "triggered"
            )
        })
    }

    fn has_effect(actions: &[IanAction], expected_name: &str) -> bool {
        actions.iter().any(|action| {
            matches!(
                action,
                IanAction::EffectPlay { name, .. } if name == expected_name
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
