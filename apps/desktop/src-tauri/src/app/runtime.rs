use crate::{
    core::{ActionDispatcher, CreatureState, EventBus},
    domain::{
        behavior::{day_phase_policy::phase_from_epoch_ms, BehaviorEngine},
        bond::{BondEngine, BondSignal},
        dialogue::{providers::DialogueSource, DialogueEngine},
        mood::{MoodEngine, MoodSignal},
        reminder::ReminderEngine,
    },
    protocol::{
        BehaviorMode, BuildTestStatus, DeveloperSnooze, DeveloperWorkspace, IanAction, IanEvent,
        IanState, PlayfulEnergy, Position, QuietHours,
    },
    security::permission::PermissionState,
    security::SecurityGate,
    storage::StorageService,
};

const BUILT_IN_ACTIVE_PET_IDS: &[&str] = &[
    "ian-adventurer",
    "ian-puppy",
    "ian-kitten",
    "ian-alpaca",
];

pub struct IanRuntime {
    event_bus: EventBus,
    state: CreatureState,
    behavior: BehaviorEngine,
    dialogue: DialogueEngine,
    mood: MoodEngine,
    bond: BondEngine,
    reminder: ReminderEngine,
    dispatcher: ActionDispatcher,
    security: SecurityGate,
    storage: StorageService,
}

impl IanRuntime {
    pub fn new(storage: StorageService) -> Self {
        let state = storage.load_state().unwrap_or_default();

        Self {
            event_bus: EventBus::default(),
            state: CreatureState::new(state),
            behavior: BehaviorEngine::default(),
            dialogue: DialogueEngine::default(),
            mood: MoodEngine::default(),
            bond: BondEngine::default(),
            reminder: ReminderEngine::default(),
            dispatcher: ActionDispatcher::default(),
            security: SecurityGate::default(),
            storage,
        }
    }

    pub fn handle_event(&mut self, event: IanEvent) -> Result<Vec<IanAction>, String> {
        self.security
            .set_permissions(PermissionState::from(self.state.snapshot()));
        self.security
            .inspect(&event)
            .map_err(|error| error.message)?;

        self.event_bus.record(event.clone());
        let _ = self.storage.record_interaction_event(&event);
        self.apply_interaction_lifecycle(&event);
        self.apply_internal_signals(&event);
        if let IanEvent::TimeTick { now_ms } = &event {
            self.state
                .set_day_phase(phase_from_epoch_ms(*now_ms).as_str().to_string());
        }

        let mut actions = match &event {
            IanEvent::DialogueUserMessage { text } => self.dialogue.reply_to(
                text.clone(),
                self.state.snapshot(),
                DialogueSource::UserBubble,
                self.mood.current(),
                self.bond.current(),
            ),
            IanEvent::TimeTick { now_ms } => {
                let mut actions = self.behavior.decide(
                    &IanEvent::TimeTick { now_ms: *now_ms },
                    self.state.snapshot(),
                );
                actions.extend(
                    self.reminder
                        .actions_for_tick(*now_ms, self.state.snapshot()),
                );
                actions
            }
            other => self.behavior.decide(other, self.state.snapshot()),
        };

        if is_user_interaction_event(&event) {
            actions.insert(
                0,
                IanAction::AppearanceScaleTo {
                    scale: 1.0,
                    duration_ms: 260,
                },
            );
        }

        self.record_life_events(&event, &actions);
        self.state.apply_actions(&actions);
        actions.push(IanAction::StateSync {
            state: self.state.snapshot().clone(),
        });
        self.dispatcher.dispatch(&actions);
        let _ = self.storage.persist_state(self.state.snapshot());

        Ok(actions)
    }

    pub fn state(&self) -> IanState {
        self.state.snapshot().clone()
    }

    pub fn save_position(&mut self, position: Position) -> Result<Vec<IanAction>, String> {
        self.state.set_position(position);
        self.storage
            .persist_state(self.state.snapshot())
            .map_err(|error| error.to_string())?;

        Ok(vec![IanAction::StateSync {
            state: self.state.snapshot().clone(),
        }])
    }

    pub fn save_behavior_mode(&mut self, mode: BehaviorMode) -> Result<IanState, String> {
        self.state.set_behavior_mode(mode);
        self.storage
            .persist_state(self.state.snapshot())
            .map_err(|error| error.to_string())?;
        Ok(self.state.snapshot().clone())
    }

    pub fn save_active_pet(&mut self, active_pet_id: String) -> Result<IanState, String> {
        if !BUILT_IN_ACTIVE_PET_IDS.contains(&active_pet_id.as_str()) {
            return Err("unknown Ian pet resource pack".to_string());
        }

        self.state.set_active_pet(active_pet_id);
        self.storage
            .persist_state(self.state.snapshot())
            .map_err(|error| error.to_string())?;
        Ok(self.state.snapshot().clone())
    }

    pub fn save_quiet_hours(&mut self, quiet_hours: QuietHours) -> Result<IanState, String> {
        self.state.set_quiet_hours(quiet_hours);
        self.storage
            .persist_state(self.state.snapshot())
            .map_err(|error| error.to_string())?;
        Ok(self.state.snapshot().clone())
    }

    pub fn save_creature_settings(
        &mut self,
        movement_intensity: String,
        bubble_frequency: String,
        rest_behavior: String,
        playful_energy: PlayfulEnergy,
        playful_snoozed_until_ms: Option<i64>,
        surface_scale: f64,
        diagnostics_enabled: bool,
    ) -> Result<IanState, String> {
        self.state.set_creature_settings(
            movement_intensity,
            bubble_frequency,
            rest_behavior,
            playful_energy,
            playful_snoozed_until_ms,
            surface_scale,
            diagnostics_enabled,
        );
        self.storage
            .persist_state(self.state.snapshot())
            .map_err(|error| error.to_string())?;
        Ok(self.state.snapshot().clone())
    }

    pub fn save_reminders_enabled(&mut self, enabled: bool) -> Result<IanState, String> {
        self.state.set_reminders_enabled(enabled);
        self.storage
            .persist_state(self.state.snapshot())
            .map_err(|error| error.to_string())?;
        Ok(self.state.snapshot().clone())
    }

    pub fn save_capability_enabled(
        &mut self,
        capability: String,
        enabled: bool,
    ) -> Result<IanState, String> {
        if !self
            .state
            .set_capability_enabled(capability.as_str(), enabled)
        {
            return Err("unknown Ian capability".to_string());
        }
        self.storage
            .persist_state(self.state.snapshot())
            .map_err(|error| error.to_string())?;
        Ok(self.state.snapshot().clone())
    }

    pub fn save_developer_workspace(
        &mut self,
        workspace: DeveloperWorkspace,
    ) -> Result<IanState, String> {
        self.state.set_developer_workspace(workspace);
        self.storage
            .persist_state(self.state.snapshot())
            .map_err(|error| error.to_string())?;
        Ok(self.state.snapshot().clone())
    }

    pub fn save_developer_snooze(&mut self, snooze: DeveloperSnooze) -> Result<IanState, String> {
        self.state.set_developer_snooze(snooze);
        self.storage
            .persist_state(self.state.snapshot())
            .map_err(|error| error.to_string())?;
        Ok(self.state.snapshot().clone())
    }

    pub fn ingest_build_test_summary(
        &mut self,
        workspace_id: Option<String>,
        tool: String,
        status: BuildTestStatus,
        duration_ms: u64,
        tests_total: u32,
        tests_failed: u32,
        error_kind: Option<String>,
    ) -> Result<Vec<IanAction>, String> {
        self.handle_event(IanEvent::DeveloperBuildTestSummary {
            workspace_id,
            tool,
            status,
            duration_ms,
            tests_total,
            tests_failed,
            error_kind,
        })
    }

    fn apply_internal_signals(&mut self, event: &IanEvent) {
        match event {
            IanEvent::MouseClick { .. } | IanEvent::MouseNear { .. } => {
                self.mood.apply(MoodSignal::UserClick);
                self.bond.apply(BondSignal::UserInteraction);
            }
            IanEvent::DialogueUserMessage { .. } => {
                self.mood.apply(MoodSignal::Dialogue);
                self.bond.apply(BondSignal::UserInteraction);
            }
            IanEvent::TimeTick { .. } => {
                self.mood.apply(MoodSignal::TimeTick);
            }
            IanEvent::ActiveAppPresence { category, .. } => {
                self.state.set_active_app_category(Some(category.clone()));
            }
            IanEvent::ScreenBounds {
                x,
                y,
                width,
                height,
            } => {
                self.state.set_screen_bounds(crate::protocol::ScreenBounds {
                    x: *x,
                    y: *y,
                    width: *width,
                    height: *height,
                });
            }
            IanEvent::DeveloperBuildTestSummary { .. }
            | IanEvent::DeveloperGitStatusChanged { .. }
            | IanEvent::KeyboardRhythm { .. }
            | IanEvent::MouseChaseCandidate { .. }
            | IanEvent::BubbleInputStarted
            | IanEvent::BubbleInputEnded => {}
            IanEvent::MouseDoubleClick { .. }
            | IanEvent::MouseDragEnd { .. }
            | IanEvent::MouseDragStart { .. }
            | IanEvent::MouseLeave { .. } => {
                self.bond.apply(BondSignal::UserInteraction);
            }
            IanEvent::AppStarted => {}
        }
    }

    fn apply_interaction_lifecycle(&mut self, event: &IanEvent) {
        if is_user_interaction_event(event) {
            self.state
                .record_user_interaction(chrono::Utc::now().timestamp_millis());
        }

        match event {
            IanEvent::MouseDragStart { .. } => self.state.set_dragging(true),
            IanEvent::MouseDragEnd { .. } => self.state.set_dragging(false),
            IanEvent::BubbleInputStarted => self.state.set_bubble_input_active(true),
            IanEvent::BubbleInputEnded | IanEvent::DialogueUserMessage { .. } => {
                self.state.set_bubble_input_active(false);
            }
            _ => {}
        }
    }

    fn record_life_events(&self, event: &IanEvent, actions: &[IanAction]) {
        let now_ms = match event {
            IanEvent::TimeTick { now_ms } => *now_ms,
            _ => chrono::Utc::now().timestamp_millis(),
        };
        let event_kind = event.event_type();

        let life_event_type = match event {
            IanEvent::AppStarted => Some("life.started"),
            IanEvent::MouseClick { .. } => Some("interaction.click"),
            IanEvent::MouseDoubleClick { .. } => Some("interaction.double_click"),
            IanEvent::DialogueUserMessage { .. } => Some("dialogue.reply"),
            _ => None,
        };

        if let Some(life_event_type) = life_event_type {
            let payload = serde_json::json!({
                "event": event_kind,
                "action_count": actions.len(),
            });
            let _ = self
                .storage
                .record_life_event(life_event_type, &payload.to_string(), now_ms);
        }

        for action in actions {
            let Some(action_event_type) = life_event_type_for_action(action) else {
                continue;
            };
            let payload = serde_json::json!({
                "event": event_kind,
                "action": action_type(action),
            });
            let _ = self
                .storage
                .record_life_event(action_event_type, &payload.to_string(), now_ms);
        }

        if self.state.snapshot().diagnostics_enabled {
            let action_types: Vec<&'static str> = actions.iter().map(action_type).collect();
            let payload = serde_json::json!({
                "event": event_kind,
                "actions": action_types,
                "reason": diagnostic_reason(event, actions),
            });
            let _ = self.storage.record_life_event(
                "diagnostic.behavior_decision",
                &payload.to_string(),
                now_ms,
            );
        }
    }
}

fn action_type(action: &IanAction) -> &'static str {
    match action {
        IanAction::AnimationPlay { .. } => "animation.play",
        IanAction::MovementMoveTo { .. } => "movement.move_to",
        IanAction::SpeechShow { .. } => "speech.show",
        IanAction::BubbleOpen => "bubble.open",
        IanAction::BubbleClose => "bubble.close",
        IanAction::BehaviorRunAround { .. } => "behavior.run_around",
        IanAction::BehaviorZoomies { .. } => "behavior.zoomies",
        IanAction::EffectPlay { .. } => "effect.play",
        IanAction::AppearanceScaleTo { .. } => "appearance.scale_to",
        IanAction::PlayfulStateSet { .. } => "playful.state",
        IanAction::PlayfulDiagnostic { .. } => "playful.diagnostic",
        IanAction::StateSync { .. } => "state.sync",
    }
}

fn is_user_interaction_event(event: &IanEvent) -> bool {
    matches!(
        event,
        IanEvent::MouseClick { .. }
            | IanEvent::MouseDoubleClick { .. }
            | IanEvent::MouseNear { .. }
            | IanEvent::MouseDragStart { .. }
            | IanEvent::MouseDragEnd { .. }
            | IanEvent::DialogueUserMessage { .. }
            | IanEvent::BubbleInputStarted
            | IanEvent::BubbleInputEnded
    )
}

fn life_event_type_for_action(action: &IanAction) -> Option<&'static str> {
    match action {
        IanAction::MovementMoveTo { .. } => Some("movement.move"),
        IanAction::BehaviorZoomies { .. } => Some("playful.zoomies"),
        IanAction::AnimationPlay { name, .. } if name == "sleep" => Some("rest.sleep"),
        IanAction::AnimationPlay { name, .. } if name == "idle" => Some("rest.wake"),
        _ => None,
    }
}

fn diagnostic_reason(event: &IanEvent, actions: &[IanAction]) -> &'static str {
    if matches!(event, IanEvent::TimeTick { .. }) && actions.is_empty() {
        "cooldown_or_suppressed"
    } else {
        "policy_action"
    }
}

#[cfg(test)]
mod tests {
    use super::IanRuntime;
    use crate::{
        protocol::{IanAction, IanEvent},
        storage::StorageService,
    };

    #[test]
    fn time_tick_can_emit_reminder_after_cooldown() {
        let mut runtime = IanRuntime::new(StorageService::in_memory());

        let first = runtime
            .handle_event(IanEvent::TimeTick { now_ms: 1_000 })
            .expect("first tick");
        let second = runtime
            .handle_event(IanEvent::TimeTick { now_ms: 5_401_000 })
            .expect("second tick");

        assert!(!first.iter().any(is_reminder_speech));
        assert!(second.iter().any(is_reminder_speech));
    }

    #[test]
    fn disabled_reminders_do_not_emit_runtime_reminders() {
        let mut runtime = IanRuntime::new(StorageService::in_memory());
        runtime
            .save_reminders_enabled(false)
            .expect("disable reminders");

        let first = runtime
            .handle_event(IanEvent::TimeTick { now_ms: 1_000 })
            .expect("first tick");
        let second = runtime
            .handle_event(IanEvent::TimeTick { now_ms: 5_401_000 })
            .expect("second tick");

        assert!(!first.iter().any(is_reminder_speech));
        assert!(!second.iter().any(is_reminder_speech));
    }

    #[test]
    fn save_active_pet_updates_current_pet_and_resource_pack() {
        let mut runtime = IanRuntime::new(StorageService::in_memory());

        let state = runtime
            .save_active_pet("ian-kitten".to_string())
            .expect("save pet");

        assert_eq!(state.active_pet_id, "ian-kitten");
        assert_eq!(state.active_resource_pack, "ian-kitten");
        assert_eq!(runtime.state().active_pet_id, "ian-kitten");
        assert_eq!(runtime.state().active_resource_pack, "ian-kitten");
    }

    #[test]
    fn save_active_pet_rejects_unknown_resource_pack() {
        let mut runtime = IanRuntime::new(StorageService::in_memory());

        let result = runtime.save_active_pet("../not-a-pack".to_string());

        assert!(result.is_err());
        assert_eq!(runtime.state().active_pet_id, "ian-puppy");
        assert_eq!(runtime.state().active_resource_pack, "ian-puppy");
    }

    #[test]
    fn interaction_lifecycle_events_suppress_autonomous_tick_actions() {
        let mut runtime = IanRuntime::new(StorageService::in_memory());

        let _ = runtime
            .handle_event(IanEvent::MouseDragStart { x: 1.0, y: 1.0 })
            .expect("drag starts");
        let dragging_tick = runtime
            .handle_event(IanEvent::TimeTick { now_ms: 90_000 })
            .expect("tick while dragging");
        let _ = runtime
            .handle_event(IanEvent::MouseDragEnd { x: 1.0, y: 1.0 })
            .expect("drag ends");
        let _ = runtime
            .handle_event(IanEvent::BubbleInputStarted)
            .expect("input starts");
        let input_tick = runtime
            .handle_event(IanEvent::TimeTick { now_ms: 90_000 })
            .expect("tick while input active");

        assert_no_autonomous_movement_or_sleep(&dragging_tick);
        assert_no_autonomous_movement_or_sleep(&input_tick);
    }

    fn is_reminder_speech(action: &IanAction) -> bool {
        matches!(
            action,
            IanAction::SpeechShow { text, .. } if text == "喝口水吧。" || text == "起来伸一下。"
        )
    }

    fn assert_no_autonomous_movement_or_sleep(actions: &[IanAction]) {
        assert!(!actions.iter().any(|action| match action {
            IanAction::MovementMoveTo { .. } => true,
            IanAction::AnimationPlay { name, .. } => name == "sleep",
            _ => false,
        }));
    }
}
