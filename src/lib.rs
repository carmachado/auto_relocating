pub mod helpers;

use std::{path::Path, process::Command};

use helpers::bar_manager::BarManager;
use helpers::folders;
use helpers::path_items::PathItems;
use indicatif::MultiProgress;
use std::thread;

use helpers::params::RelocateParams;

pub fn configure() {
    RelocateParams::configure()
}

pub fn export() {
    let params = RelocateParams::get_from_file();
    let params_str = params.to_string();

    print!("{} {} {}", "auto_relocating", "import", params_str);
}

pub fn import(args: &Vec<String>) -> PathItems {
    if args.len() < 3 {
        panic!("You didn't input anything to import");
    }

    let (params, path_items) = RelocateParams::get_from_args(args);
    params.save_to_file();
    path_items
}

pub fn get_path_items() -> Vec<String> {
    RelocateParams::get_from_file().path_items
}

pub fn relocate_all_folders(path_items: PathItems) {
    let params = RelocateParams::get_from_file();

    let progress = MultiProgress::new();

    let folders = folders::get_folders(&params);

    let mut relocates = Vec::with_capacity(folders.len());

    for directory in folders {
        let bar = BarManager::new(&progress);
        let path = path_items.clone();

        relocates.push(thread::spawn(move || relocate_folder(directory, path, bar)));
    }

    for relocate in relocates {
        relocate.join().unwrap();
    }
}

fn relocate_folder(directory: String, path_items: PathItems, bar: BarManager) {
    bar.start();

    if Path::new(&directory).is_dir() {
        let output = Command::new("svn")
            .arg("relocate")
            .arg(&path_items.from)
            .arg(&path_items.to)
            .arg(&directory)
            .output()
            .expect("Failed to execute process");

        let error = String::from_utf8_lossy(&output.stderr);

        bar.finish(&directory, &error);
    } else {
        bar.finish(&directory, &"Directory not found");
    }
}
