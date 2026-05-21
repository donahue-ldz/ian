use crate::protocol::{BehaviorMode, CurrentBehavior, IanAction, IanState, Position};

pub struct CreatureState {
    state: IanState,
}

impl CreatureState {
    pub fn new(state: IanState) -> Self {
        Self { state }
    }

    pub fn snapshot(&self) -> &IanState {
        &self.state
    }

    pub fn set_position(&mut self, position: Position) {
        self.state.position = position.clone();
        self.state.home_anchor = position;
    }

    pub fn set_behavior_mode(&mut self, mode: BehaviorMode) {
        self.state.behavior_mode = mode;
    }

    pub fn set_reminders_enabled(&mut self, enabled: bool) {
        self.state.reminders_enabled = enabled;
    }

    pub fn set_capability_enabled(&mut self, capability: &str, enabled: bool) -> bool {
        match capability {
            "byom" => self.state.byom_enabled = enabled,
            "git_metadata" => self.state.git_metadata_enabled = enabled,
            "build_test_events" => self.state.build_test_events_enabled = enabled,
            "keyboard_rhythm" => self.state.keyboard_rhythm_enabled = enabled,
            "active_app_presence" => self.state.active_app_presence_enabled = enabled,
            _ => return false,
        }
        true
    }

    pub fn apply_actions(&mut self, actions: &[IanAction]) {
        for action in actions {
            match action {
                IanAction::AnimationPlay { name, .. } => {
                    self.state.current_animation = name.clone();
                    self.state.current_behavior = match name.as_str() {
                        "walk" => CurrentBehavior::Walking,
                        "happy" => CurrentBehavior::Happy,
                        "run" => CurrentBehavior::Running,
                        "sleep" => CurrentBehavior::Sleeping,
                        _ => CurrentBehavior::Idle,
                    };
                }
                IanAction::BehaviorRunAround { .. } => {
                    self.state.current_animation = "run".to_string();
                    self.state.current_behavior = CurrentBehavior::Running;
                }
                IanAction::MovementMoveTo { x, y, .. } => {
                    self.state.position = Position { x: *x, y: *y };
                }
                IanAction::StateSync { state } => {
                    self.state = state.clone();
                }
                IanAction::SpeechShow { .. } | IanAction::BubbleOpen | IanAction::BubbleClose => {}
            }
        }
    }
}
