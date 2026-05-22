use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug)]
pub struct SecretStore {
    key_path: PathBuf,
}

impl Default for SecretStore {
    fn default() -> Self {
        Self {
            key_path: PathBuf::new(),
        }
    }
}

impl SecretStore {
    pub fn new(app_dir: &Path) -> Self {
        Self {
            key_path: app_dir.join("secrets").join("byom_api_key"),
        }
    }

    pub fn is_available(&self) -> bool {
        !self.key_path.as_os_str().is_empty()
    }

    pub fn save_api_key(&self, api_key: &str) -> io::Result<()> {
        if let Some(parent) = self.key_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&self.key_path, api_key)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&self.key_path, fs::Permissions::from_mode(0o600))?;
        }
        Ok(())
    }

    pub fn read_api_key(&self) -> io::Result<Option<String>> {
        if !self.key_path.exists() {
            return Ok(None);
        }

        fs::read_to_string(&self.key_path).map(Some)
    }

    pub fn delete_api_key(&self) -> io::Result<()> {
        match fs::remove_file(&self.key_path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error),
        }
    }

    pub fn redacted_status(&self) -> io::Result<String> {
        let Some(api_key) = self.read_api_key()? else {
            return Ok("empty".to_string());
        };

        Ok(format!("configured:{}", redact_secret(&api_key)))
    }
}

fn redact_secret(secret: &str) -> String {
    if secret.chars().count() <= 8 {
        return "********".to_string();
    }

    let prefix: String = secret.chars().take(4).collect();
    let suffix: String = secret
        .chars()
        .rev()
        .take(4)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    format!("{prefix}...{suffix}")
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::SecretStore;

    #[test]
    fn api_key_store_saves_reads_deletes_and_redacts_without_config() {
        let dir = tempdir().expect("temp dir");
        let store = SecretStore::new(dir.path());

        store.save_api_key("sk-test-secret").expect("save key");

        assert_eq!(
            store.read_api_key().expect("read key").as_deref(),
            Some("sk-test-secret")
        );
        assert_eq!(
            store.redacted_status().expect("status"),
            "configured:sk-t...cret"
        );

        store.delete_api_key().expect("delete key");
        assert!(store.read_api_key().expect("read deleted").is_none());
    }
}
