use crate::protocol::{IanError, IanEvent};

#[derive(Default)]
pub struct RateLimiter;

impl RateLimiter {
    pub fn allow(&self, _event: &IanEvent) -> Result<(), IanError> {
        Ok(())
    }
}
