use crate::protocol::{IanAction, IanEvent};

const MOMENTARY_WINDOW_MS: i64 = 10 * 60 * 1000;

#[derive(Debug, Default)]
pub struct MomentaryLifeState {
    last_seen_ms: i64,
    last_wake_ms: Option<i64>,
    last_affection_ms: Option<i64>,
    last_run_end_ms: Option<i64>,
    last_drag_end_ms: Option<i64>,
    ignored_since_ms: Option<i64>,
}

impl MomentaryLifeState {
    pub fn record_event(&mut self, event: &IanEvent) {
        match event {
            IanEvent::TimeTick { now_ms } => {
                self.last_seen_ms = *now_ms;
                self.expire(*now_ms);
            }
            IanEvent::AppStarted => {
                self.last_wake_ms = Some(self.last_seen_ms);
                self.ignored_since_ms = Some(self.last_seen_ms);
            }
            IanEvent::MouseClick { .. } => {
                self.last_affection_ms = Some(self.last_seen_ms);
                self.ignored_since_ms = None;
            }
            IanEvent::MouseDragEnd { .. } => {
                self.last_drag_end_ms = Some(self.last_seen_ms);
                self.ignored_since_ms = None;
            }
            IanEvent::MouseDoubleClick { .. } => {
                self.ignored_since_ms = None;
            }
            _ => {}
        }
    }

    pub fn record_actions(&mut self, actions: &[IanAction]) {
        if actions.iter().any(|action| {
            matches!(
                action,
                IanAction::BehaviorRunAround { .. } | IanAction::BehaviorZoomies { .. }
            )
        }) {
            self.last_run_end_ms = Some(self.last_seen_ms);
        }
    }

    pub fn consume_recent_wake_phrase(&mut self) -> Option<&'static str> {
        let now_ms = self.last_seen_ms;
        if self.is_recent(self.last_wake_ms, now_ms) {
            self.last_wake_ms = None;
            return Some("刚醒。");
        }

        None
    }

    pub fn is_recently_affectionate(&self) -> bool {
        self.is_recent(self.last_affection_ms, self.last_seen_ms)
    }

    pub fn is_recently_dragged(&self) -> bool {
        self.is_recent(self.last_drag_end_ms, self.last_seen_ms)
    }

    pub fn is_recently_run_settling(&self) -> bool {
        self.is_recent(self.last_run_end_ms, self.last_seen_ms)
    }

    fn expire(&mut self, now_ms: i64) {
        if !self.is_recent(self.last_wake_ms, now_ms) {
            self.last_wake_ms = None;
        }
        if !self.is_recent(self.last_affection_ms, now_ms) {
            self.last_affection_ms = None;
        }
        if !self.is_recent(self.last_run_end_ms, now_ms) {
            self.last_run_end_ms = None;
        }
        if !self.is_recent(self.last_drag_end_ms, now_ms) {
            self.last_drag_end_ms = None;
        }
        if !self.is_recent(self.ignored_since_ms, now_ms) {
            self.ignored_since_ms = Some(now_ms);
        }
    }

    fn is_recent(&self, timestamp_ms: Option<i64>, now_ms: i64) -> bool {
        timestamp_ms
            .map(|timestamp| now_ms >= timestamp && now_ms - timestamp <= MOMENTARY_WINDOW_MS)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use crate::protocol::IanEvent;

    use super::MomentaryLifeState;

    #[test]
    fn wake_state_is_short_lived_and_consumed_once() {
        let mut state = MomentaryLifeState::default();

        state.record_event(&IanEvent::AppStarted);

        assert_eq!(state.consume_recent_wake_phrase(), Some("刚醒。"));
        assert_eq!(state.consume_recent_wake_phrase(), None);
    }

    #[test]
    fn momentary_state_expires_after_window() {
        let mut state = MomentaryLifeState::default();

        state.record_event(&IanEvent::AppStarted);
        state.record_event(&IanEvent::TimeTick { now_ms: 601_000 });

        assert_eq!(state.consume_recent_wake_phrase(), None);
    }
}
