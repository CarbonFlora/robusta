use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Robusta")]
#[command(author="Zi Hao L.", version="0.0.1", about="CAD Plan Checker Utility.", long_about = None)]

pub struct RobustaArgs {
    /// Optionally select a dxf file to be parsed through.
    /// This feature is currently not implemented.
    #[arg(required = false)]
    pub input_file: Option<String>,
}