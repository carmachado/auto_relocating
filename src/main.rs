use std::env;

use auto_relocating::{
    adapters::dialoguer_adapter::DialoguerAdapter, controllers::path_items::PathItems,
};

static HELP_TEXT: &'static str = include_str!("./assets/help.md");

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        return match args[1].as_str() {
            "help" => println!("{}", HELP_TEXT),
            "config" => auto_relocating::configure(),
            "run" => run(&args),
            "export" => auto_relocating::export(),
            "import" => {
                auto_relocating::import(&args);
            }
            _ => println!("Error: Command not found.\nUse help command to see the command list."),
        };
    }

    run(&args);
    DialoguerAdapter::select("Press enter to exit", &["Exit"]);
}

fn run(args: &Vec<String>) {
    let mut path_items = PathItems::default();

    if has_params(args) {
        path_items = auto_relocating::import(&args);
    }

    if path_items.is_empty() {
        path_items = get_from_to_selectdlg()
    };

    let path_items = normalize_with_file(path_items);

    auto_relocating::relocate(path_items);
}

fn has_params(args: &Vec<String>) -> bool {
    args.len() >= 3
}

fn normalize_with_file(path_items: PathItems) -> PathItems {
    let path_items_file = auto_relocating::get_path_items_from_file();

    path_items.normalized(&path_items_file)
}

fn get_from_to_selectdlg() -> PathItems {
    let path_items = auto_relocating::get_path_items_from_file();
    let vec_path_items = vec![path_items.from, path_items.to];

    let selection: usize =
        DialoguerAdapter::select("Where do you want to relocate?", &vec_path_items);

    PathItems::new(&String::new(), &vec_path_items[selection])
}
