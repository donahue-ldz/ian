use crate::{
    core::{ActionDispatcher, CreatureState, EventBus},
    domain::{
        behavior::BehaviorEngine,
        bond::{BondEngine, BondSignal},
        dialogue::{providers::DialogueSource, DialogueEngine},
        mood::{MoodEngine, MoodSignal},
    },
    protocol::{BehaviorMode, IanAction, IanEvent, IanState, Position},
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
            dispatcher: ActionDispatcher::default(),
            security: SecurityGate::default(),
            storage,
        }
    }

    pub fn handle_event(&mut self, event: IanEvent) -> Result<Vec<IanAction>, String> {
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
            IanEvent::MouseDoubleClick { .. }
            | IanEvent::MouseDragEnd { .. }
            | IanEvent::MouseDragStart { .. } => {
                self.bond.apply(BondSignal::UserInteraction);
            }
            IanEvent::AppStarted => {}
        }
    }
}
