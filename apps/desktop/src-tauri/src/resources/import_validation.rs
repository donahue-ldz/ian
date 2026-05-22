#[derive(Debug, Default)]
pub struct ResourcePackImportValidator;

impl ResourcePackImportValidator {
    pub fn validate_manifest(
        &self,
        id: &str,
        version: &str,
        assets: &[&str],
    ) -> Result<(), String> {
        if id.trim().is_empty() || version.trim().is_empty() {
            return Err("resource pack id and version are required".to_string());
        }
        for asset in assets {
            validate_local_static_asset(asset)?;
            if asset.ends_with(".js")
                || asset.ends_with(".mjs")
                || asset.ends_with(".ts")
                || asset.ends_with(".sh")
            {
                return Err("resource pack imports cannot contain scripts".to_string());
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct SoundPackPolicy {
    enabled_by_default: bool,
    max_volume: f32,
}

impl Default for SoundPackPolicy {
    fn default() -> Self {
        Self {
            enabled_by_default: false,
            max_volume: 0.6,
        }
    }
}

impl SoundPackPolicy {
    pub fn enabled_by_default(&self) -> bool {
        self.enabled_by_default
    }

    pub fn normalize_volume(&self, requested: f32) -> f32 {
        requested.clamp(0.0, self.max_volume)
    }

    pub fn can_play(&self, user_enabled: bool, do_not_disturb: bool) -> bool {
        user_enabled && !do_not_disturb
    }

    pub fn validate_asset(&self, asset: &str) -> Result<(), String> {
        validate_local_static_asset(asset)?;
        if !(asset.ends_with(".ogg") || asset.ends_with(".wav") || asset.ends_with(".mp3")) {
            return Err("sound pack asset must be an audio file".to_string());
        }
        Ok(())
    }
}

fn validate_local_static_asset(asset: &str) -> Result<(), String> {
    if asset.starts_with("http://")
        || asset.starts_with("https://")
        || asset.contains("..")
        || asset.starts_with('/')
        || asset.contains('\\')
    {
        return Err("resource asset must be a local static relative path".to_string());
    }
    Ok(())
}

#[derive(Debug, Default)]
pub struct ResourceImportProductGate;

impl ResourceImportProductGate {
    pub fn is_product_ready(&self) -> bool {
        false
    }

    pub fn requirements(&self) -> Vec<String> {
        vec![
            "manifest 校验、资源完整性校验和用户可见回滚必须先完成".to_string(),
            "脚本、远端引用和路径穿越必须在导入入口统一拒绝".to_string(),
            "导入失败必须保持当前 resource pack 可恢复且不破坏桌面壳".to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::{ResourceImportProductGate, ResourcePackImportValidator, SoundPackPolicy};
    use crate::resources::resource_registry::MultiMonitorRoamingPolicy;

    #[test]
    fn imported_resource_packs_reject_scripts_and_remote_references() {
        let validator = ResourcePackImportValidator::default();

        assert!(validator
            .validate_manifest("ian-local", "0.1.0", &["sprite.png", "animations.json"])
            .is_ok());
        assert!(validator
            .validate_manifest("ian-bad", "0.1.0", &["sprite.js"])
            .is_err());
        assert!(validator
            .validate_manifest("ian-remote", "0.1.0", &["https://example.com/sprite.png"])
            .is_err());
    }

    #[test]
    fn optional_sound_pack_is_off_bounded_and_local_only() {
        let policy = SoundPackPolicy::default();

        assert!(!policy.enabled_by_default());
        assert_eq!(policy.normalize_volume(1.0), 0.6);
        assert!(!policy.can_play(false, true));
        assert!(!policy.can_play(true, true));
        assert!(policy.can_play(true, false));
        assert!(policy.validate_asset("sounds/chirp.ogg").is_ok());
        assert!(policy
            .validate_asset("https://example.com/chirp.ogg")
            .is_err());
    }

    #[test]
    fn resource_import_gate_blocks_productization_until_required_safety_checks_exist() {
        let gate = ResourceImportProductGate::default();

        assert!(!gate.is_product_ready());
        assert!(gate
            .requirements()
            .iter()
            .any(|requirement| requirement.contains("manifest") && requirement.contains("回滚")));
        assert!(gate
            .requirements()
            .iter()
            .any(|requirement| requirement.contains("脚本") && requirement.contains("远端引用")));
    }

    #[test]
    fn multi_monitor_roaming_stays_inside_current_safe_area_without_opt_in() {
        let policy = MultiMonitorRoamingPolicy::default();

        assert!(!policy.cross_monitor_enabled());
        assert_eq!(
            policy.clamp_target(900.0, 700.0, 800.0, 600.0),
            (760.0, 560.0)
        );
    }
}
