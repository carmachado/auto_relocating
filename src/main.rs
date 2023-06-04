use std::env;

use auto_relocating::helpers::{dlg_helper, path_items::PathItems};

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
    dlg_helper::select("Press enter to exit", &["Exit"]);
}

fn run(args: &Vec<String>) {
    let path_args = if args.len() >= 3 {
        auto_relocating::import(&args)
    } else {
        PathItems::default()
    };

    let path_items = if path_args.is_empty() {
        get_from_to_selectdlg()
    } else {
        path_args
    };

    let path_items = get_stable_path_items(path_items);

    auto_relocating::relocate_all_folders(path_items);
}

fn get_stable_path_items(path_items: PathItems) -> PathItems {
    let path_file = auto_relocating::get_path_items();
    let path_file = PathItems::new(&path_file[0], &path_file[1]);

    path_items.normalized(&path_file)
}

fn get_from_to_selectdlg() -> PathItems {
    let path_items = auto_relocating::get_path_items();

    let selection: usize = dlg_helper::select("Where do you want to relocate?", &path_items);

    PathItems::new(&String::new(), &path_items[selection])
}
