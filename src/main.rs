use std::env;

use auto_relocating::helpers::dlg_helper;

static HELP_TEXT: &'static str = include_str!("./assets/help.md");

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        return match args[1].as_str() {
            "help" => println!("{}", HELP_TEXT),
            "config" => auto_relocating::configure(),
            "run" => run(),
            "export" => auto_relocating::export(),
            _ => println!("Error: Command not found.\nUse help command to see the command list."),
        };
    }

    run();
    dlg_helper::select("Press enter to exit", &["Exit"]);
}

fn run() {
    let path_items = auto_relocating::get_path_items();

    let selection: usize = dlg_helper::select("Where do you want to relocate?", &path_items);

    let folder_from = if selection == 0 {
        &path_items[1]
    } else {
        &path_items[0]
    };

    auto_relocating::relocate_all_folders(&folder_from, &path_items[selection]);
}
