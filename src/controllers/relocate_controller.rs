use std::{
    process::{Command, Output},
    thread::{self, JoinHandle},
};

use crate::adapters::{
    multi_progress_adapter::MultiProgressAdaper, progress_bar_adapter::ProgressBarAdapter,
};

use super::{folders, params::RelocateParams, params_reader::file_reader, path_items::PathItems};

pub struct RelocateController {
    multi_progress: MultiProgressAdaper,
    directories: Vec<String>,
    path_items: PathItems,
}

impl RelocateController {
    pub fn new(path_items: PathItems) -> RelocateController {
        let params = file_reader::get_from_file().unwrap_or_else(|_| RelocateParams::default());

        RelocateController {
            directories: folders::get_folders(&params),
            multi_progress: MultiProgressAdaper::new(),
            path_items,
        }
    }

    pub fn run(&self) {
        let threads: Vec<JoinHandle<()>> = self
            .directories
            .clone()
            .into_iter()
            .map(|directory| self.create_relocate_thread(&directory))
            .collect();
        threads
            .into_iter()
            .for_each(|thread| thread.join().unwrap())
    }

    fn create_relocate_thread(&self, directory: &String) -> JoinHandle<()> {
        let progress_bar = self.multi_progress.create_stylized_bar(2);
        let path_items_clone = self.path_items.clone();
        let directory_clone = directory.clone();

        thread::spawn(move || {
            RelocateController::relocate_directory(directory_clone, path_items_clone, progress_bar)
        })
    }

    fn relocate_directory(directory: String, path_items: PathItems, bar: ProgressBarAdapter) {
        bar.start();

        if folders::is_directory(&directory) {
            let output = RelocateController::run_svn_relocate(&directory, &path_items);

            let error = match output {
                Ok(out) => String::from_utf8_lossy(&out.stderr).to_string(),
                Err(error) => error.to_string(),
            };

            bar.finish(&directory, &error);
        } else {
            bar.finish(&directory, &"Directory not found");
        }
    }

    fn run_svn_relocate(
        directory: &String,
        path_items: &PathItems,
    ) -> Result<Output, std::io::Error> {
        Command::new("svn")
            .arg("relocate")
            .arg(&path_items.from)
            .arg(&path_items.to)
            .arg(&directory)
            .output()
    }
}
