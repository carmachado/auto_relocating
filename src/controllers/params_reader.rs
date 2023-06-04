pub mod arg_reader {

    use crate::controllers::{params::RelocateParams, path_items::PathItems};

    use super::file_reader;

    struct ArgReader {
        params: RelocateParams,
        orginal_path_items: Vec<String>,
        path_items: PathItems,
    }

    impl ArgReader {
        fn new() -> Self {
            let params = match file_reader::get_from_file() {
                Ok(params) => params,
                Err(_) => RelocateParams::default(),
            };

            ArgReader {
                orginal_path_items: params.path_items.clone(),
                params,
                path_items: PathItems::default(),
            }
        }
    }

    pub fn get_from_args(args: &Vec<String>) -> (RelocateParams, PathItems) {
        let mut reader = ArgReader::new();

        let mut i = 2;
        while i + 1 < args.len() {
            let next_arg = args[i + 1].clone();

            match args[i].as_str() {
                "--path" => reader.params.folder_read = next_arg,
                "--from" => {
                    reader.params.path_items[0] = next_arg.clone();
                    reader.path_items.from = next_arg;
                }
                "--to" => {
                    reader.params.path_items[1] = next_arg.clone();
                    reader.path_items.to = next_arg;
                }
                "--deep" => reader.params.deep_search = next_arg.parse().unwrap(),
                _ => panic!("Invalid param"),
            }

            i += 2;
        }

        if reader.params.path_items[0] == reader.params.path_items[1] {
            reader.params.path_items = reader.orginal_path_items;
        }

        (reader.params, reader.path_items)
    }
}

pub mod file_reader {
    use std::{
        fs::File,
        io::{self, BufReader},
        path::Path,
    };

    use crate::controllers::params::{self, RelocateParams};

    pub fn has_file() -> bool {
        Path::new(&params::get_file_name()).is_file()
    }

    pub fn get_from_file() -> io::Result<RelocateParams> {
        let file = File::open(params::get_file_name())?;

        let reader = BufReader::new(file);

        let deserialized: RelocateParams = serde_json::from_reader(reader).unwrap();

        Ok(deserialized)
    }
}
