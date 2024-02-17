// use std::io::Write;
use crate::wrapper::RobustaEntities;

use crate::*;

pub fn open_from_path(path: PathBuf) -> Drawing {
    let drawing = Drawing::load_file(path);
    match drawing {
        Ok(d) => d,
        Err(_e) => Drawing::new(),
    }
}

pub fn parse_dxf(path: &Option<String>) -> RobustaEntities {
    let d = open_from_path(path.clone().unwrap_or_default().into());
    RobustaEntities::from(&d)
}
