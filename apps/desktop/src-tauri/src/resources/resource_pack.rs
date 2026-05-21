use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePackManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub species: String,
}
