pub mod adapters;
pub mod controllers;

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
    let params_str = params.to_string();

    print!("{} {} {}", "auto_relocating", "import", params_str);
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
    file_reader::get_from_file().unwrap().as_path_items()
}
