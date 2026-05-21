use std::path::PathBuf;

pub struct ResourceLoader {
    root: PathBuf,
}

impl ResourceLoader {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn pet_path(&self, pet_id: &str) -> PathBuf {
        self.root.join("pets").join(pet_id)
    }
}
