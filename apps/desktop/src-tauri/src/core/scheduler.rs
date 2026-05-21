use chrono::Utc;

use crate::protocol::{BehaviorMode, IanAction, IanEvent, IanState};

const DESKTOP_TICK_WINDOW_SECS: i64 = 15;

#[derive(Default)]
pub struct Scheduler;

impl Scheduler {
    pub fn tick(&self) -> IanEvent {
        IanEvent::TimeTick {
            now_ms: Utc::now().timestamp_millis(),
        }
    }
}

#[derive(Default)]
pub struct BehaviorScheduler;

impl BehaviorScheduler {
    pub fn action_for_tick(&self, now_ms: i64, state: &IanState) -> Option<IanAction> {
        if state.current_animation == "run" || state.is_dragging || state.is_bubble_input_active {
            return None;
        }

        let second = now_ms.div_euclid(1000);
        let cadence = match state.behavior_mode {
            BehaviorMode::Quiet => 120,
            BehaviorMode::Normal => 90,
            BehaviorMode::Lively => 45,
        };

        if is_in_tick_window(second, cadence) {
            return Some(IanAction::AnimationPlay {
                name: "sleep".to_string(),
                looped: true,
            });
        }

        if !matches!(state.behavior_mode, BehaviorMode::Quiet) && is_in_tick_window(second, 45) {
            return Some(IanAction::AnimationPlay {
                name: "rest".to_string(),
                looped: true,
            });
        }

        Some(IanAction::AnimationPlay {
            name: "idle".to_string(),
            looped: true,
        })
    }
}

fn is_in_tick_window(second: i64, cadence: i64) -> bool {
    second >= cadence && second.rem_euclid(cadence) < DESKTOP_TICK_WINDOW_SECS
}

#[cfg(test)]
mod tests {
    use super::BehaviorScheduler;
    use crate::protocol::{BehaviorMode, IanAction, IanState};

    #[test]
    fn behavior_mode_controls_tick_frequency() {
        let scheduler = BehaviorScheduler::default();
        let mut state = IanState::default();
        state.behavior_mode = BehaviorMode::Quiet;

        let quiet = scheduler.action_for_tick(45_000, &state);
        state.behavior_mode = BehaviorMode::Lively;
        let lively = scheduler.action_for_tick(45_000, &state);

        assert!(matches!(
            quiet,
            Some(IanAction::AnimationPlay { ref name, .. }) if name == "idle"
        ));
        assert!(matches!(
            lively,
            Some(IanAction::AnimationPlay { ref name, .. }) if name == "sleep"
        ));
    }

    #[test]
    fn cadence_windows_survive_fifteen_second_desktop_ticks() {
        let scheduler = BehaviorScheduler::default();
        let state = IanState::default();

        let rest = scheduler.action_for_tick(52_000, &state);
        let sleep = scheduler.action_for_tick(97_000, &state);

        assert!(matches!(
            rest,
            Some(IanAction::AnimationPlay { ref name, .. }) if name == "rest"
        ));
        assert!(matches!(
            sleep,
            Some(IanAction::AnimationPlay { ref name, .. }) if name == "sleep"
        ));
    }

    #[test]
    fn scheduler_does_not_interrupt_run_animation() {
        let scheduler = BehaviorScheduler::default();
        let mut state = IanState::default();
        state.current_animation = "run".to_string();

        assert!(scheduler.action_for_tick(90_000, &state).is_none());
    }

    #[test]
    fn scheduler_does_not_interrupt_active_user_interaction() {
        let scheduler = BehaviorScheduler::default();
        let mut state = IanState::default();
        state.is_dragging = true;
        assert!(scheduler.action_for_tick(90_000, &state).is_none());

        state.is_dragging = false;
        state.is_bubble_input_active = true;
        assert!(scheduler.action_for_tick(90_000, &state).is_none());
    }
}
