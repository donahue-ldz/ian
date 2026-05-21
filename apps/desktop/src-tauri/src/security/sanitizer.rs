use crate::protocol::{IanError, IanEvent};

#[derive(Default)]
pub struct Sanitizer;

impl Sanitizer {
    pub fn inspect(&self, event: &IanEvent) -> Result<(), IanError> {
        if let IanEvent::DialogueUserMessage { text } = event {
            if text.chars().count() > 256 {
                return Err(IanError::new(
                    "dialogue_payload_too_large",
                    "Dialogue message is too long for P0 bubble input.",
                ));
            }
        }

        Ok(())
    }
}
