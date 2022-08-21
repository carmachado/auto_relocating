use std::path::PathBuf;

use super::params::RelocateParams;
use glob::{glob, GlobError};

fn get_parent_folder(e: Result<PathBuf, GlobError>) -> Option<String> {
    if let Ok(path) = e {
        return Some(path.parent()?.to_str()?.to_owned());
    }
    None
}

pub fn get_folders(params: &RelocateParams) -> Vec<String> {
    let deep_char = match params.deep_search {
        true => "**",
        false => "*",
    };

    let pattern = params.folder_read.to_string() + deep_char + "\\.svn";

    glob(pattern.as_str())
        .expect("Failed to read glob pattern")
        .filter_map(get_parent_folder)
        .collect()
}
