use crate::protocol::IanEvent;

use super::{PerceptionAdapter, SensitivityLevel};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyboardRhythmSummary {
    pub window_ms: u64,
    pub intensity: String,
    pub count: u32,
}

#[derive(Default)]
pub struct KeyboardRhythmAdapter {
    enabled: bool,
    queued: Vec<KeyboardRhythmSummary>,
}

impl KeyboardRhythmAdapter {
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        self.queued.clear();
    }

    pub fn push_summary(&mut self, summary: KeyboardRhythmSummary) {
        self.queued.push(summary);
    }
}

impl PerceptionAdapter for KeyboardRhythmAdapter {
    fn id(&self) -> &'static str {
        "keyboard.rhythm"
    }

    fn sensitivity(&self) -> SensitivityLevel {
        SensitivityLevel::High
    }

    fn poll(&mut self) -> Vec<IanEvent> {
        if !self.enabled {
            self.queued.clear();
            return Vec::new();
        }

        std::mem::take(&mut self.queued)
            .into_iter()
            .map(|summary| IanEvent::KeyboardRhythm {
                window_ms: summary.window_ms,
                intensity: summary.intensity,
                count: summary.count,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{KeyboardRhythmAdapter, KeyboardRhythmSummary};
    use crate::{adapters::PerceptionAdapter, protocol::IanEvent};

    #[test]
    fn keyboard_rhythm_stops_collecting_after_disable() {
        let mut adapter = KeyboardRhythmAdapter::default();
        adapter.enable();
        adapter.disable();
        adapter.push_summary(KeyboardRhythmSummary {
            window_ms: 60_000,
            intensity: "active".to_string(),
            count: 100,
        });

        assert!(adapter.poll().is_empty());
    }

    #[test]
    fn keyboard_rhythm_emits_only_window_intensity_and_count() {
        let mut adapter = KeyboardRhythmAdapter::default();
        adapter.enable();
        adapter.push_summary(KeyboardRhythmSummary {
            window_ms: 60_000,
            intensity: "active".to_string(),
            count: 100,
        });

        let events = adapter.poll();

        assert!(matches!(
            events.first(),
            Some(IanEvent::KeyboardRhythm {
                window_ms: 60_000,
                intensity,
                count: 100,
            }) if intensity == "active"
        ));
    }
}
