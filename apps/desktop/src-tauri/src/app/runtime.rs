use crate::{
    core::{ActionDispatcher, CreatureState, EventBus},
    domain::{
        behavior::BehaviorEngine,
        bond::{BondEngine, BondSignal},
        dialogue::{providers::DialogueSource, DialogueEngine},
        mood::{MoodEngine, MoodSignal},
        reminder::ReminderEngine,
    },
    protocol::{BehaviorMode, IanAction, IanEvent, IanState, Position},
    security::permission::PermissionState,
    security::SecurityGate,
    storage::StorageService,
};

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
        self.apply_internal_signals(&event);

        let mut actions = match event {
            IanEvent::DialogueUserMessage { text } => self.dialogue.reply_to(
                text,
                self.state.snapshot(),
                DialogueSource::UserBubble,
                self.mood.current(),
                self.bond.current(),
            ),
            IanEvent::TimeTick { now_ms } => {
                let mut actions = self
                    .behavior
                    .decide(&IanEvent::TimeTick { now_ms }, self.state.snapshot());
                actions.extend(
                    self.reminder
                        .actions_for_tick(now_ms, self.state.snapshot()),
                );
                actions
            }
            other => self.behavior.decide(&other, self.state.snapshot()),
        };

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
            IanEvent::DeveloperBuildTestSummary { .. }
            | IanEvent::DeveloperGitStatusChanged { .. }
            | IanEvent::KeyboardRhythm { .. }
            | IanEvent::ActiveAppPresence { .. } => {}
            IanEvent::MouseDoubleClick { .. }
            | IanEvent::MouseDragEnd { .. }
            | IanEvent::MouseDragStart { .. } => {
                self.bond.apply(BondSignal::UserInteraction);
            }
            IanEvent::AppStarted => {}
        }
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

    fn is_reminder_speech(action: &IanAction) -> bool {
        matches!(
            action,
            IanAction::SpeechShow { text, .. } if text == "喝口水吧。" || text == "起来伸一下。"
        )
    }
}
