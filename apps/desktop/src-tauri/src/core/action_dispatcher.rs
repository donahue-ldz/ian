use crate::protocol::IanAction;

#[derive(Default)]
pub struct ActionDispatcher {
    last_batch_len: usize,
}

impl ActionDispatcher {
    pub fn dispatch(&mut self, actions: &[IanAction]) {
        self.last_batch_len = actions.len();
    }

    pub fn last_batch_len(&self) -> usize {
        self.last_batch_len
    }
}
