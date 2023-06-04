use std::{fs::File, io::Write};

use crate::adapters::dialoguer_adapter::DialoguerAdapter;

use super::params::{self, RelocateParams};

pub fn configure() -> RelocateParams {
    let mut params = RelocateParams::default();

    params.folder_read = DialoguerAdapter::input(
        "Enter the directory that you want to scan (You can scan multiples using ; as separator)",
    );

    params.deep_search = DialoguerAdapter::confirm("You want to scan subdirectories?");

    params.path_items = vec![
        DialoguerAdapter::input("Enter the origin repository"),
        DialoguerAdapter::input("Enter the destination repository"),
    ];

    save_to_file(&params);

    params
}

pub fn save_to_file(params: &RelocateParams) {
    let json = serde_json::to_string(&params).unwrap();

    let mut file = File::create(params::get_file_name()).unwrap();

    file.write_all(json.as_bytes()).unwrap();
}
