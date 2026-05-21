use super::mood_state::MoodState;
use super::MoodSignal;

pub struct MoodEngine {
    state: MoodState,
}

impl Default for MoodEngine {
    fn default() -> Self {
        Self {
            state: MoodState::Calm,
        }
    }
}

impl MoodEngine {
    pub fn current(&self) -> MoodState {
        self.state
    }

    pub fn apply(&mut self, signal: MoodSignal) -> MoodState {
        self.state = match signal {
            MoodSignal::UserClick | MoodSignal::Dialogue => MoodState::Happy,
            MoodSignal::TimeTick => match self.state {
                MoodState::Happy => MoodState::Calm,
                MoodState::Calm => MoodState::Sleepy,
                MoodState::Sleepy | MoodState::Bored => MoodState::Sleepy,
            },
        };

        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::MoodEngine;
    use crate::domain::mood::{MoodSignal, MoodState};

    #[test]
    fn click_and_dialogue_make_mood_happy_then_tick_softens_it() {
        let mut engine = MoodEngine::default();

        assert_eq!(engine.apply(MoodSignal::UserClick), MoodState::Happy);
        assert_eq!(engine.apply(MoodSignal::TimeTick), MoodState::Calm);
        assert_eq!(engine.apply(MoodSignal::Dialogue), MoodState::Happy);
    }
}
