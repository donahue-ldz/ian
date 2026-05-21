use chrono::Utc;

use crate::protocol::IanEvent;

#[derive(Default)]
pub struct Scheduler;

impl Scheduler {
    pub fn tick(&self) -> IanEvent {
        IanEvent::TimeTick {
            now_ms: Utc::now().timestamp_millis(),
        }
    }
}
