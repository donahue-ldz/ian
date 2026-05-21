use super::mood_state::MoodState;

#[derive(Default)]
pub struct MoodEngine;

impl MoodEngine {
    pub fn current(&self) -> MoodState {
        MoodState::Calm
    }
}
