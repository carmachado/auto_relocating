use auto_relocating::controllers::{params::RelocateParams, relocate_controller};

use clap::Parser;

fn main() {
    let params = RelocateParams::parse();
    relocate_controller::run(params);
}
