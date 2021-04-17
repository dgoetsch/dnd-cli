use crate::domain::character::Character;
use anyhow::Result;

pub struct Store {
    storage_dir: String,
}

impl Store {
    pub fn new(storage_dir: &str) -> Result<Store> {
        let store = std::fs::create_dir_all(storage_dir).map(|_| Store {
            storage_dir: storage_dir.to_string(),
        })?;
        Ok(store)
    }

    fn path_for(&self, key: String) -> String {
        self.storage_dir
            .strip_suffix('/')
            .map(|s| s.to_string())
            .unwrap_or(self.storage_dir.clone())
            + "/"
            + key.strip_prefix('/').map(|s| s).unwrap_or(key.as_str())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Store {
    pub fn load_character(&self, name: String) -> Result<Character> {
        let content = std::fs::read_to_string(self.path_for(format!("characters/{}.json", name)))?;
        let character = serde_json::from_str(content.as_str())?;
        Ok(character)
    }
}
