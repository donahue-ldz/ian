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

#[derive(Debug)]
pub struct MultiMonitorRoamingPolicy {
    cross_monitor_enabled: bool,
    margin: f64,
}

impl Default for MultiMonitorRoamingPolicy {
    fn default() -> Self {
        Self {
            cross_monitor_enabled: false,
            margin: 40.0,
        }
    }
}

impl MultiMonitorRoamingPolicy {
    pub fn cross_monitor_enabled(&self) -> bool {
        self.cross_monitor_enabled
    }

    pub fn clamp_target(
        &self,
        x: f64,
        y: f64,
        screen_width: f64,
        screen_height: f64,
    ) -> (f64, f64) {
        (
            x.clamp(0.0, (screen_width - self.margin).max(0.0)),
            y.clamp(0.0, (screen_height - self.margin).max(0.0)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::ResourceRegistry;

    #[test]
    fn validates_minimum_resource_pack_contract() {
        let registry = ResourceRegistry::default();

        assert!(registry.validate_pack("ian-alpaca", "0.1.0").is_ok());
        assert!(registry.validate_pack("", "0.1.0").is_err());
        assert!(registry.validate_pack("ian-alpaca", "").is_err());
    }
}
