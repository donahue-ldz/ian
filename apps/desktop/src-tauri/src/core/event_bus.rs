use crate::protocol::IanEvent;

#[derive(Default)]
pub struct EventBus {
    recent: Vec<IanEvent>,
}

impl EventBus {
    pub fn record(&mut self, event: IanEvent) {
        self.recent.push(event);
        if self.recent.len() > 64 {
            self.recent.remove(0);
        }
    }

    pub fn recent(&self) -> &[IanEvent] {
        &self.recent
    }
}
