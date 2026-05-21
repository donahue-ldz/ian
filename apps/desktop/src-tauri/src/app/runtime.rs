use crate::{
    core::{ActionDispatcher, CreatureState, EventBus},
    domain::{
        behavior::BehaviorEngine,
        dialogue::{providers::DialogueSource, DialogueEngine},
    },
    protocol::{IanAction, IanEvent, IanState, Position},
    security::SecurityGate,
    storage::StorageService,
};

pub struct IanRuntime {
    event_bus: EventBus,
    state: CreatureState,
    behavior: BehaviorEngine,
    dialogue: DialogueEngine,
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

        let mut actions = match event {
            IanEvent::DialogueUserMessage { text } => {
                self.dialogue
                    .reply_to(text, self.state.snapshot(), DialogueSource::UserBubble)
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
}
