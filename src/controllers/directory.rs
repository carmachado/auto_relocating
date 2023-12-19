use std::path::{Path, PathBuf};

use glob::{glob, GlobError};

fn get_parent_folder(e: Result<PathBuf, GlobError>) -> Option<String> {
    if let Ok(path) = e {
        return Some(path.parent()?.to_str()?.to_owned());
    }
    None
}

pub fn get_folders(directories: &String) -> Vec<String> {
    let pattern = get_directories_pattern(&directories);

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

fn get_directories_pattern(directories: &String) -> Vec<String> {
    directories
        .split(';')
        .map(|folder| Path::new(folder).join(".svn").to_str().unwrap().to_string())
        .collect()
}

pub fn is_directory(directory: &String) -> bool {
    Path::new(&directory).is_dir()
}

#[cfg(test)]
mod tests {
    use crate::controllers::directory;
    use std::path::MAIN_SEPARATOR;

    use super::*;

    #[test]
    fn get_directories_pattern_nofolder() {
        assert_eq!(
            directory::get_directories_pattern(&"".to_string()),
            vec![format!(".svn")]
        )
    }

    #[test]
    fn get_directories_pattern_1folder() {
        assert_eq!(
            directory::get_directories_pattern(&"folder1".to_string()),
            vec![format!("folder1{MAIN_SEPARATOR}.svn")]
        )
    }

    #[test]
    fn get_directories_pattern_2folder() {
        assert_eq!(
            directory::get_directories_pattern(&"folder1;folder2".to_string()),
            vec![
                format!("folder1{MAIN_SEPARATOR}.svn"),
                format!("folder2{MAIN_SEPARATOR}.svn")
            ]
        )
    }

    #[test]
    fn get_folders_nofolder() {
        let path1 = Path::new("testno");
        std::fs::create_dir_all(&path1).unwrap();

        let directories = format!("testno{MAIN_SEPARATOR}").to_string();

        assert!(directory::get_folders(&directories).is_empty());
    }

    #[test]
    fn get_folders_1folder() {
        let path2 = Path::new("test2").join("test").join(".svn");
        std::fs::create_dir_all(&path2).unwrap();

        let directories = format!("test2{MAIN_SEPARATOR}*").to_string();

        assert_eq!(
            directory::get_folders(&directories),
            vec![path2.parent().unwrap().to_str().unwrap()]
        );
    }

    #[test]
    fn get_folders_2folder() {
        let path1 = Path::new("test").join(".svn");
        let path2 = Path::new("test2").join("test").join(".svn");

        std::fs::create_dir_all(&path1).unwrap();
        std::fs::create_dir_all(&path2).unwrap();

        let directories = format!("*;test2{MAIN_SEPARATOR}*").to_string();

        assert_eq!(
            directory::get_folders(&directories),
            vec![
                path1.parent().unwrap().to_str().unwrap(),
                path2.parent().unwrap().to_str().unwrap()
            ]
        );
    }
}
