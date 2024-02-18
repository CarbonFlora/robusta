// use std::io::Write;
use crate::wrapper::RFile;

use crate::*;

pub fn open_from_path(path: PathBuf) -> Drawing {
    let drawing = Drawing::load_file(path);
    match drawing {
        Ok(d) => d,
        Err(_e) => Drawing::new(),
    }
}

pub fn parse_dxf(path: &Option<String>) -> RFile {
    let d = open_from_path(path.clone().unwrap_or_default().into());
    RFile::from(&d)
}
