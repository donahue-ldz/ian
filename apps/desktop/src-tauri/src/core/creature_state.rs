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
        self.state.position = position;
    }

    pub fn set_behavior_mode(&mut self, mode: BehaviorMode) {
        self.state.behavior_mode = mode;
    }

    pub fn apply_actions(&mut self, actions: &[IanAction]) {
        for action in actions {
            match action {
                IanAction::AnimationPlay { name, .. } => {
                    self.state.current_animation = name.clone();
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
