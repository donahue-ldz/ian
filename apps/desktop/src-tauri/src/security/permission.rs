use crate::protocol::{IanError, IanEvent};

#[derive(Default)]
pub struct PermissionGate;

impl PermissionGate {
    pub fn allow(&self, event: &IanEvent) -> Result<(), IanError> {
        match event {
            IanEvent::AppStarted
            | IanEvent::TimeTick { .. }
            | IanEvent::MouseClick { .. }
            | IanEvent::MouseDoubleClick { .. }
            | IanEvent::MouseDragStart { .. }
            | IanEvent::MouseDragEnd { .. }
            | IanEvent::DialogueUserMessage { .. } => Ok(()),
        }
    }
}
