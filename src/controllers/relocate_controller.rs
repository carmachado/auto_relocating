use std::{
    process::{Command, Output},
    thread::{self},
};

use crate::adapters::{
    multi_progress_adapter::MultiProgressAdaper, progress_bar_adapter::ProgressBarAdapter,
};

use super::{directory, params::RelocateParams};

pub fn run(params: RelocateParams) {
    let directories = directory::get_folders(&params.directories);
    let multi_progress = MultiProgressAdaper::new();

    thread::scope(|s| {
        directories.into_iter().for_each(|directory: String| {
            let progress_bar = multi_progress.create_stylized_bar(2);
            s.spawn(|| relocate_directory(directory, &params, progress_bar));
        });
    });
}

fn relocate_directory(directory: String, params: &RelocateParams, bar: ProgressBarAdapter) {
    bar.start();

    if directory::is_directory(&directory) {
        let output = run_svn_relocate(&directory, &params.from, &params.to);

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
    from: &String,
    to: &String,
) -> Result<Output, std::io::Error> {
    Command::new("svn")
        .arg("relocate")
        .arg(&from)
        .arg(&to)
        .arg(&directory)
        .output()
}
