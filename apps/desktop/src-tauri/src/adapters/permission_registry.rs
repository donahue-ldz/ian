#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterSensitivity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdapterPermissionDescriptor {
    pub id: &'static str,
    pub sensitivity: AdapterSensitivity,
    pub enabled_by_default: bool,
    pub consent_copy: &'static str,
}

#[derive(Debug, Clone)]
pub struct AdapterPermissionRegistry {
    descriptors: Vec<AdapterPermissionDescriptor>,
}

impl Default for AdapterPermissionRegistry {
    fn default() -> Self {
        Self {
            descriptors: vec![
                AdapterPermissionDescriptor {
                    id: "time.tick",
                    sensitivity: AdapterSensitivity::Low,
                    enabled_by_default: true,
                    consent_copy: "Uses local time only.",
                },
                AdapterPermissionDescriptor {
                    id: "mouse.boundary",
                    sensitivity: AdapterSensitivity::Low,
                    enabled_by_default: true,
                    consent_copy: "Uses pointer position near Ian only.",
                },
                AdapterPermissionDescriptor {
                    id: "developer.git_metadata",
                    sensitivity: AdapterSensitivity::Medium,
                    enabled_by_default: false,
                    consent_copy: "Uses branch, dirty flag, and short commit hash only.",
                },
                AdapterPermissionDescriptor {
                    id: "developer.build_test",
                    sensitivity: AdapterSensitivity::Medium,
                    enabled_by_default: false,
                    consent_copy: "Uses aggregate build and test results only.",
                },
                AdapterPermissionDescriptor {
                    id: "keyboard.rhythm",
                    sensitivity: AdapterSensitivity::High,
                    enabled_by_default: false,
                    consent_copy: "Uses aggregate typing rhythm, never key content.",
                },
                AdapterPermissionDescriptor {
                    id: "active_app.presence",
                    sensitivity: AdapterSensitivity::High,
                    enabled_by_default: false,
                    consent_copy: "Uses app category only, not titles or document names.",
                },
                AdapterPermissionDescriptor {
                    id: "desktop.window",
                    sensitivity: AdapterSensitivity::High,
                    enabled_by_default: false,
                    consent_copy: "Uses explicit desktop shell signals only.",
                },
            ],
        }
    }
}

impl AdapterPermissionRegistry {
    pub fn get(&self, id: &str) -> Option<&AdapterPermissionDescriptor> {
        self.descriptors
            .iter()
            .find(|descriptor| descriptor.id == id)
    }

    pub fn is_allowed_by_default(&self, id: &str) -> bool {
        self.get(id)
            .map(|descriptor| descriptor.enabled_by_default)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::{AdapterPermissionRegistry, AdapterSensitivity};

    #[test]
    fn registry_declares_adapter_sensitivity_and_defaults_unknown_to_denied() {
        let registry = AdapterPermissionRegistry::default();

        let keyboard = registry
            .get("keyboard.rhythm")
            .expect("keyboard descriptor");
        assert_eq!(keyboard.sensitivity, AdapterSensitivity::High);
        assert!(!keyboard.enabled_by_default);
        assert!(!registry.is_allowed_by_default("clipboard.full_text"));
    }
}
