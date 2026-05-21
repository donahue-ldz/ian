#[derive(Default)]
pub struct ResourceRegistry {
    active_pet_id: String,
}

impl ResourceRegistry {
    pub fn active_pet_id(&self) -> &str {
        if self.active_pet_id.is_empty() {
            "ian-alpaca"
        } else {
            &self.active_pet_id
        }
    }
}
