use dirs_next;
use std::{
    fs::{self, File},
    io::{BufReader, Write},
    path::Path,
    fmt
};

use serde;

use super::dlg_helper;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RelocateParams {
    pub path_items: Vec<String>,
    pub folder_read: String,
    pub deep_search: bool,
}

impl Default for RelocateParams {
    fn default() -> Self {
        RelocateParams {
            path_items: vec![],
            folder_read: String::new(),
            deep_search: false,
        }
    }
}


impl fmt::Display for RelocateParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, 
               "--path \"{}\" --from \"{}\" --to \"{}\"  --deep \"{}\"", 
               self.folder_read, 
               self.path_items[0], 
               self.path_items[1],
               self.deep_search
        )
    }
}

impl RelocateParams {
    fn get_file_name() -> String {
        let mut path = dirs_next::config_dir().unwrap();
        path.push("auto_relocating");

        fs::create_dir_all(&path).unwrap();
        path.push("auto_relocating.json");

        path.to_string_lossy().to_string()
    }

    pub fn get_from_file() -> RelocateParams {
        if !Path::new(RelocateParams::get_file_name().as_str()).is_file() {
            RelocateParams::configure();
        }

        let file =
            File::open(RelocateParams::get_file_name()).expect("Configuration file not found");

        let reader = BufReader::new(file);

        let deserialized: RelocateParams = serde_json::from_reader(reader).unwrap();

        deserialized
    }

    pub fn configure() {
        let mut params = RelocateParams::default();

        params.folder_read = dlg_helper::input("Enter the directory that you want to scan (The parent folder from child working copies)");

        params.deep_search = dlg_helper::confirm("You want to scan subdirectories?");

        params.path_items = vec![
            dlg_helper::input("Enter the origin repository"),
            dlg_helper::input("Enter the destination repository"),
        ];

        let json = serde_json::to_string(&params).unwrap();

        let mut file = File::create(RelocateParams::get_file_name()).unwrap();

        file.write_all(json.as_bytes()).unwrap();
    }
}
