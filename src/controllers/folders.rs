use std::path::{Path, PathBuf};

use super::params::RelocateParams;
use glob::{glob, GlobError};

fn get_parent_folder(e: Result<PathBuf, GlobError>) -> Option<String> {
    if let Ok(path) = e {
        return Some(path.parent()?.to_str()?.to_owned());
    }
    None
}

pub fn get_folders(params: &RelocateParams) -> Vec<String> {
    let pattern = get_directories_pattern(&params);

    glob(pattern.as_str())
        .expect("Failed to read glob pattern")
        .filter_map(get_parent_folder)
        .collect()
}

fn get_directories_pattern(params: &RelocateParams) -> String {
    let deep_char = match params.deep_search {
        true => "**",
        false => "*",
    };

    let folder_filter =
        deep_char.to_string() + std::path::MAIN_SEPARATOR.to_string().as_str() + ".svn";

    params
        .folder_read
        .to_string()
        .split(';')
        .map(|folder| folder.to_string() + folder_filter.as_str())
        .reduce(|prev, atu| prev + ";" + atu.as_str())
        .unwrap()
}

pub fn is_directory(directory: &String) -> bool {
    Path::new(&directory).is_dir()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_directories_pattern_nofolder() {
        let mut params = RelocateParams::default();
        params.folder_read = "".to_string();

        assert_eq!(super::get_directories_pattern(&params), "*\\.svn")
    }

    #[test]
    fn get_directories_pattern_1folder() {
        let mut params = RelocateParams::default();
        params.folder_read = "folder1".to_string();

        assert_eq!(super::get_directories_pattern(&params), "folder1*\\.svn")
    }

    #[test]
    fn get_directories_pattern_2folder() {
        let mut params = RelocateParams::default();
        params.folder_read = "folder1;folder2".to_string();

        assert_eq!(
            super::get_directories_pattern(&params),
            "folder1*\\.svn;folder2*\\.svn"
        )
    }

    #[test]
    fn get_directories_pattern_deep() {
        let mut params = RelocateParams::default();
        params.folder_read = "".to_string();
        params.deep_search = true;

        assert_eq!(super::get_directories_pattern(&params), "**\\.svn")
    }
}
