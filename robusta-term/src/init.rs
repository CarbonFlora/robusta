use crate::args::RobustaArgs;
use robusta_2d::app::app2d;

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
    app2d();
}

fn integrated_init() {
    app2d();
}
