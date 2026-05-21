use chrono::Utc;

use crate::protocol::IanEvent;

use super::{PerceptionAdapter, SensitivityLevel};

#[derive(Default)]
pub struct TimeAdapter;

impl PerceptionAdapter for TimeAdapter {
    fn id(&self) -> &'static str {
        "time"
    }

    fn sensitivity(&self) -> SensitivityLevel {
        SensitivityLevel::Low
    }

    fn poll(&mut self) -> Vec<IanEvent> {
        vec![IanEvent::TimeTick {
            now_ms: Utc::now().timestamp_millis(),
        }]
    }
}
