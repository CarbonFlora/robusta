use crate::args::RobustaArgs;
use robusta_gui::two::startup_bevy_2d;

use anyhow::Result;
use clap::Parser;

pub fn init() -> Result<()> {
    let args = RobustaArgs::parse();
    match args.input_file {
        None => integrated_init(),
        Some(file_name) => using_dxf_init(file_name),
    }
    Ok(())
}

fn using_dxf_init(_file_name: String) {
    startup_bevy_2d();
}

fn integrated_init() {
    startup_bevy_2d();
}
