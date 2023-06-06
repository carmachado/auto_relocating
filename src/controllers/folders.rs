use std::path::{Path, PathBuf, MAIN_SEPARATOR};

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

    pattern
        .into_iter()
        .map(|pattern| {
            glob(pattern.as_str())
                .expect("Failed to read glob pattern")
                .filter_map(get_parent_folder)
        })
        .flatten()
        .collect()
}

fn get_directories_pattern(params: &RelocateParams) -> Vec<String> {
    let deep_char = match params.deep_search {
        true => "**",
        false => "*",
    };

    let folder_filter = format!("{deep_char}{MAIN_SEPARATOR}.svn");

    params
        .folder_read
        .to_string()
        .split(';')
        .map(|folder| folder.to_string() + folder_filter.as_str())
        .collect()
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

        assert_eq!(
            super::get_directories_pattern(&params),
            vec![format!("*{MAIN_SEPARATOR}.svn")]
        )
    }

    #[test]
    fn get_directories_pattern_1folder() {
        let mut params = RelocateParams::default();
        params.folder_read = "folder1".to_string();

        assert_eq!(
            super::get_directories_pattern(&params),
            vec![format!("folder1*{MAIN_SEPARATOR}.svn")]
        )
    }

    #[test]
    fn get_directories_pattern_2folder() {
        let mut params = RelocateParams::default();
        params.folder_read = "folder1;folder2".to_string();

        assert_eq!(
            super::get_directories_pattern(&params),
            vec![
                format!("folder1*{MAIN_SEPARATOR}.svn"),
                format!("folder2*{MAIN_SEPARATOR}.svn")
            ]
        )
    }

    #[test]
    fn get_directories_pattern_deep() {
        let mut params = RelocateParams::default();
        params.folder_read = "".to_string();
        params.deep_search = true;

        assert_eq!(
            super::get_directories_pattern(&params),
            vec![format!("**{MAIN_SEPARATOR}.svn")]
        )
    }

    #[test]
    fn get_folders_nofolder() {
        let path1 = Path::new("testno");

        std::fs::create_dir_all(&path1).unwrap();

        let mut params = RelocateParams::default();
        params.folder_read = format!("testno{MAIN_SEPARATOR}").to_string();

        assert!(super::get_folders(&params).is_empty());
    }

    #[test]
    fn get_folders_1folder() {
        let path2 = Path::new("test2").join("test").join(".svn");

        std::fs::create_dir_all(&path2).unwrap();

        let mut params = RelocateParams::default();
        params.folder_read = format!("test2{MAIN_SEPARATOR}").to_string();

        assert_eq!(
            super::get_folders(&params),
            vec![path2.parent().unwrap().to_str().unwrap()]
        );
    }

    #[test]
    fn get_folders_2folder() {
        let path1 = Path::new("test").join(".svn");
        let path2 = Path::new("test2").join("test").join(".svn");

        std::fs::create_dir_all(&path1).unwrap();
        std::fs::create_dir_all(&path2).unwrap();

        let mut params = RelocateParams::default();
        params.folder_read = format!(";test2{MAIN_SEPARATOR}").to_string();

        assert_eq!(
            super::get_folders(&params),
            vec![
                path1.parent().unwrap().to_str().unwrap(),
                path2.parent().unwrap().to_str().unwrap()
            ]
        );
    }

    #[test]
    fn get_folders_2folder_deep() {
        let path1 = Path::new("test").join(".svn");
        let path2 = Path::new("test2").join("test").join(".svn");

        std::fs::create_dir_all(&path1).unwrap();
        std::fs::create_dir_all(&path2).unwrap();

        let mut params = RelocateParams::default();
        params.folder_read = "".to_string();
        params.deep_search = true;

        assert_eq!(
            super::get_folders(&params),
            vec![
                path1.parent().unwrap().to_str().unwrap(),
                path2.parent().unwrap().to_str().unwrap()
            ]
        );
    }
}
