#[derive(Default)]
pub struct SecretStore;

impl SecretStore {
    pub fn is_available(&self) -> bool {
        false
    }
}
