use super::reminder_policy::ReminderPolicy;
use crate::protocol::{IanAction, IanState};

#[derive(Default)]
pub struct ReminderEngine {
    policy: ReminderPolicy,
    last_reminder_ms: Option<i64>,
}

impl ReminderEngine {
    pub fn is_enabled(&self) -> bool {
        self.policy.enabled
    }

    pub fn actions_for_tick(&mut self, now_ms: i64, state: &IanState) -> Vec<IanAction> {
        if !self.policy.enabled
            || !state.reminders_enabled
            || state.current_animation == "run"
            || self
                .policy
                .should_reduce_for_app_category(state.active_app_category.as_deref())
        {
            return Vec::new();
        }

        let Some(last_reminder_ms) = self.last_reminder_ms else {
            self.last_reminder_ms = Some(now_ms);
            return Vec::new();
        };

        if now_ms - last_reminder_ms < self.policy.cooldown_ms {
            return Vec::new();
        }

        self.last_reminder_ms = Some(now_ms);
        vec![
            IanAction::BubbleOpen,
            IanAction::SpeechShow {
                text: reminder_text(now_ms).to_string(),
                mood: Some("calm".to_string()),
                duration_ms: Some(4_000),
            },
            IanAction::AnimationPlay {
                name: "happy".to_string(),
                looped: false,
            },
        ]
    }
}

fn reminder_text(now_ms: i64) -> &'static str {
    let cycle = (now_ms / ReminderPolicy::default().cooldown_ms) % 2;
    if cycle == 0 {
        "喝口水吧。"
    } else {
        "起来伸一下。"
    }
}

#[cfg(test)]
mod tests {
    use super::ReminderEngine;
    use crate::protocol::{IanAction, IanState};

    #[test]
    fn first_tick_arms_cooldown_without_showing_reminder() {
        let mut engine = ReminderEngine::default();
        let state = IanState::default();

        let actions = engine.actions_for_tick(1_000, &state);

        assert!(actions.is_empty());
    }

    #[test]
    fn reminder_appears_after_cooldown_and_then_cools_down() {
        let mut engine = ReminderEngine::default();
        let state = IanState::default();

        assert!(engine.actions_for_tick(1_000, &state).is_empty());
        let actions = engine.actions_for_tick(5_401_000, &state);
        let cooldown_actions = engine.actions_for_tick(5_402_000, &state);

        assert!(matches!(actions.first(), Some(IanAction::BubbleOpen)));
        assert!(actions.iter().any(|action| matches!(
            action,
            IanAction::SpeechShow { text, .. } if text == "起来伸一下。"
        )));
        assert!(cooldown_actions.is_empty());
    }

    #[test]
    fn disabled_reminders_do_not_emit_actions() {
        let mut engine = ReminderEngine::default();
        let mut state = IanState::default();
        state.reminders_enabled = false;

        assert!(engine.actions_for_tick(1_000, &state).is_empty());
        assert!(engine.actions_for_tick(5_401_000, &state).is_empty());
    }
}
