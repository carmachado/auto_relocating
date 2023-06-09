use std::fs;

use super::path_items::PathItems;
use serde;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RelocateParams {
    pub path_items: Vec<String>,
    pub folder_read: String,
    pub deep_search: bool,
}

impl Default for RelocateParams {
    fn default() -> Self {
        RelocateParams {
            path_items: vec!["".to_string(), "".to_string()],
            folder_read: String::new(),
            deep_search: false,
        }
    }
}

impl RelocateParams {
    pub fn as_path_items(&self) -> PathItems {
        PathItems::new(&self.path_items[0], &self.path_items[1])
    }
}

pub fn get_file_name() -> String {
    let mut path = dirs_next::config_dir().unwrap();
    path.push("auto_relocating");

    fs::create_dir_all(&path).unwrap();
    path.push("auto_relocating.json");

    path.to_string_lossy().to_string()
}
