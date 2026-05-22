use std::collections::{HashMap, HashSet};

const ALLOWED_PLUGIN_PERMISSIONS: &[&str] = &["time.tick", "mouse.boundary", "dialogue.demo"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub permissions: Vec<String>,
    pub events: Vec<String>,
    pub actions: Vec<String>,
    pub enabled_by_default: bool,
}

impl PluginManifest {
    pub fn validate(&self) -> Result<(), String> {
        if self.id.trim().is_empty()
            || self.name.trim().is_empty()
            || self.version.trim().is_empty()
        {
            return Err("plugin manifest id, name, and version are required".to_string());
        }
        if self.enabled_by_default {
            return Err("plugin system is default-off".to_string());
        }
        for permission in &self.permissions {
            if !ALLOWED_PLUGIN_PERMISSIONS.contains(&permission.as_str()) {
                return Err(format!(
                    "unknown or high-sensitive plugin permission: {permission}"
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct LocalPluginPermissionModel {
    authorized_events: HashMap<String, HashSet<String>>,
}

impl LocalPluginPermissionModel {
    pub fn authorize(&mut self, plugin_id: &str, events: &[&str]) {
        self.authorized_events.insert(
            plugin_id.to_string(),
            events.iter().map(|event| event.to_string()).collect(),
        );
    }

    pub fn revoke(&mut self, plugin_id: &str) {
        self.authorized_events.remove(plugin_id);
    }

    pub fn allow_event(&self, plugin_id: &str, event: &str) -> Result<(), String> {
        let Some(events) = self.authorized_events.get(plugin_id) else {
            return Err("plugin is not authorized".to_string());
        };
        if events.contains(event) {
            Ok(())
        } else {
            Err("plugin event is not authorized".to_string())
        }
    }
}

#[derive(Debug, Default)]
pub struct LocalPluginRuntimeGate;

impl LocalPluginRuntimeGate {
    pub fn can_load_plugins(&self) -> bool {
        false
    }

    pub fn can_execute_scripts(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::{LocalPluginPermissionModel, LocalPluginRuntimeGate, PluginManifest};

    #[test]
    fn plugin_manifest_validates_permissions_and_stays_default_off() {
        let manifest = PluginManifest {
            id: "local-companion-clock".to_string(),
            name: "Companion Clock".to_string(),
            version: "0.1.0".to_string(),
            permissions: vec!["time.tick".to_string()],
            events: vec!["time.tick".to_string()],
            actions: vec![],
            enabled_by_default: false,
        };

        assert!(manifest.validate().is_ok());
        assert!(!manifest.enabled_by_default);

        let bad = PluginManifest {
            permissions: vec!["clipboard.full_text".to_string()],
            ..manifest
        };
        assert!(bad.validate().is_err());
        assert!(!LocalPluginRuntimeGate::default().can_load_plugins());
        assert!(!LocalPluginRuntimeGate::default().can_execute_scripts());
    }

    #[test]
    fn plugin_permission_model_requires_authorization_and_revokes_immediately() {
        let mut model = LocalPluginPermissionModel::default();

        assert!(model.allow_event("plugin-1", "time.tick").is_err());
        model.authorize("plugin-1", &["time.tick"]);
        assert!(model.allow_event("plugin-1", "time.tick").is_ok());
        model.revoke("plugin-1");
        assert!(model.allow_event("plugin-1", "time.tick").is_err());
    }
}
