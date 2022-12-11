use dirs_next;
use std::{
    fmt,
    fs::{self, File},
    io::{self, BufReader, Write},
    path::Path,
};

use super::{dlg_helper, path_items::PathItems};
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

impl fmt::Display for RelocateParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "--path \"{}\" --from \"{}\" --to \"{}\" --deep \"{}\"",
            self.folder_read, self.path_items[0], self.path_items[1], self.deep_search
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

    fn read_from_file() -> io::Result<RelocateParams> {
        let file = File::open(RelocateParams::get_file_name())?;

        let reader = BufReader::new(file);

        let deserialized: RelocateParams = serde_json::from_reader(reader).unwrap();

        Ok(deserialized)
    }

    pub fn save_to_file(&self) {
        let json = serde_json::to_string(&self).unwrap();

        let mut file = File::create(RelocateParams::get_file_name()).unwrap();

        file.write_all(json.as_bytes()).unwrap();
    }

    pub fn get_from_args(args: &Vec<String>) -> (RelocateParams, PathItems) {
        let mut params = match RelocateParams::read_from_file() {
            Ok(params) => params,
            Err(_) => RelocateParams::default(),
        };

        let before_path_items = params.path_items.clone();
        let mut path_items = PathItems::default();

        let mut i = 2;
        while i + 1 < args.len() {
            let next_arg = args[i + 1].clone();

            match args[i].as_str() {
                "--path" => params.folder_read = next_arg,
                "--from" => {
                    params.path_items[0] = next_arg.clone();
                    path_items.from = next_arg;
                }
                "--to" => {
                    params.path_items[1] = next_arg.clone();
                    path_items.to = next_arg;
                }
                "--deep" => params.deep_search = next_arg.parse().unwrap(),
                _ => panic!("Invalid param"),
            }

            i += 2;
        }

        if params.path_items[0] == params.path_items[1] {
            params.path_items = before_path_items;
        }

        (params, path_items)
    }

    pub fn get_from_file() -> RelocateParams {
        if !Path::new(RelocateParams::get_file_name().as_str()).is_file() {
            RelocateParams::configure();
        }

        RelocateParams::read_from_file().expect("Configration file not found")
    }

    pub fn configure() {
        let mut params = RelocateParams::default();

        params.folder_read = dlg_helper::input("Enter the directory that you want to scan (The parent folder from child working copies)");

        params.deep_search = dlg_helper::confirm("You want to scan subdirectories?");

        params.path_items = vec![
            dlg_helper::input("Enter the origin repository"),
            dlg_helper::input("Enter the destination repository"),
        ];

        params.save_to_file();
    }
}
