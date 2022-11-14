pub mod helpers;

use std::{path::Path, process::Command};

use helpers::bar_manager::BarManager;
use helpers::folders;
use indicatif::MultiProgress;
use std::thread;

use helpers::params::RelocateParams;

pub fn configure() {
    RelocateParams::configure()
}

pub fn export() {
    let result = "auto_relocating import ".to_string()
        + RelocateParams::get_from_file().to_string().as_str();

    print!("{}", result.trim());
}

pub fn import(args: &Vec<String>) {
    if args.len() < 3 {
        panic!("You didn't input anything to import");
    }

    let params = RelocateParams::get_from_args(args);
    params.save_to_file();
}

pub fn get_path_items() -> Vec<String> {
    RelocateParams::get_from_file().path_items
}

pub fn relocate_all_folders(folder_from: &String, folder_to: &String) {
    let params = RelocateParams::get_from_file();

    let progress = MultiProgress::new();

    let folders = folders::get_folders(&params);

    let mut relocates = Vec::with_capacity(folders.len());

    for directory in folders {
        let bar = BarManager::new(&progress);

        let folder_from = folder_from.clone();
        let folder_to = folder_to.clone();

        relocates.push(thread::spawn(move || {
            relocate_folder(directory, folder_from, folder_to, bar)
        }));
    }

    for relocate in relocates {
        relocate.join().unwrap();
    }
}

fn relocate_folder(directory: String, folder_from: String, folder_to: String, bar: BarManager) {
    bar.start();

    if Path::new(&directory).is_dir() {
        let output = Command::new("svn")
            .arg("relocate")
            .arg(&folder_from)
            .arg(&folder_to)
            .arg(&directory)
            .output()
            .expect("Failed to execute process");

        let error = String::from_utf8_lossy(&output.stderr);

        bar.finish(&directory, &error);
    } else {
        bar.finish(&directory, &"Directory not found");
    }
}
