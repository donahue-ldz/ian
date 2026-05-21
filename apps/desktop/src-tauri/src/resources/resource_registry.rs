#[derive(Default)]
pub struct ResourceRegistry {
    active_pet_id: String,
}

impl ResourceRegistry {
    pub fn active_pet_id(&self) -> &str {
        if self.active_pet_id.is_empty() {
            "ian-kitten"
        } else {
            &self.active_pet_id
        }
    }

    pub fn validate_pack(&self, id: &str, version: &str) -> Result<(), String> {
        if id.trim().is_empty() {
            return Err("resource pack id is required".to_string());
        }

        if version.trim().is_empty() {
            return Err("resource pack version is required".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::ResourceRegistry;

    #[test]
    fn validates_minimum_resource_pack_contract() {
        let registry = ResourceRegistry::default();

        assert_eq!(registry.active_pet_id(), "ian-kitten");
        assert!(registry.validate_pack("ian-alpaca", "0.1.0").is_ok());
        assert!(registry.validate_pack("", "0.1.0").is_err());
        assert!(registry.validate_pack("ian-alpaca", "").is_err());
    }
}
