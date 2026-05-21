pub mod permission;
pub mod rate_limiter;
pub mod sanitizer;

use crate::protocol::{IanError, IanEvent};

#[derive(Default)]
pub struct SecurityGate {
    permissions: permission::PermissionGate,
    rate_limiter: rate_limiter::RateLimiter,
    sanitizer: sanitizer::Sanitizer,
}

impl SecurityGate {
    pub fn inspect(&mut self, event: &IanEvent) -> Result<(), IanError> {
        self.permissions.allow(event)?;
        self.rate_limiter.allow(event)?;
        self.sanitizer.inspect(event)
    }
}
