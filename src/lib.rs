pub mod adapters;
pub mod controllers;

use std::process::Command;

use controllers::params_reader::arg_reader;
use controllers::params_writer;
use controllers::path_items::PathItems;
use controllers::relocate_controller::RelocateController;

use crate::controllers::params_reader::file_reader;

pub fn configure() {
    params_writer::configure();
}

pub fn export() {
    let params = file_reader::get_from_file().expect("You don't have data to be exported");

    let mut command = Command::new("auto_relocating");
    command.args([
        "import",
        "--path",
        params.folder_read.as_str(),
        "--from",
        params.path_items[0].as_str(),
        "--to",
        params.path_items[1].as_str(),
        "--deep",
        params.deep_search.to_string().as_str(),
    ]);

    print!("{:?}", command);
}

pub fn import(args: &Vec<String>) -> PathItems {
    if args.len() < 3 {
        panic!("You didn't input anything to import");
    }

    let (params, path_items) = arg_reader::get_from_args(args);
    params_writer::save_to_file(&params);
    path_items
}

pub fn relocate(path_items: PathItems) {
    let relocate_controller = RelocateController::new(path_items);

    relocate_controller.run();
}

pub fn get_path_items_from_file() -> PathItems {
    match file_reader::get_from_file() {
        Ok(file) => file.as_path_items(),
        Err(_) => params_writer::configure().as_path_items(),
    }
}
