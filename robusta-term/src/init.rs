use crate::args::RobustaArgs;

use anyhow::Result;
use clap::Parser;
use robusta_gui::app::app2d;

// This is currently setup to only handle dxf files.
pub fn init() -> Result<()> {
    let args = RobustaArgs::parse();
    match args.input_file {
        None => integrated_init(),
        Some(file_name) => using_dxf_init(file_name),
    }
    Ok(())
}

pub fn using_dxf_init(file_name: String) {
    app2d(Some(file_name));
}

fn integrated_init() {
    app2d(None);
}
