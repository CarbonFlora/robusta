use crate::args::RobustaArgs;
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

fn using_dxf_init(file_name: String) {
    todo!();
}

fn integrated_init() {
    todo!();
}
