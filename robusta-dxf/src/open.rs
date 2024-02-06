// use std::io::Write;
use crate::wrapper::DXFWrapper;

use crate::*;

pub fn open_from_path(path: PathBuf) -> Drawing {
    let drawing = Drawing::load_file(path);
    match drawing {
        Ok(d) => return d,
        Err(_e) => return Drawing::new(),
    };

    // todo!() As I currently do not know how to do conditional debug compilation, uncomment when you want to see what's in the file at this point.
    // let mut w = Vec::new();
    // for entity in drawing.entities() {
    //     writeln!(&mut w, "Found: {entity:?}")?;
    // }
    // return Ok(drawing);
}

pub fn parse_dxf(path: &Option<String>) -> DXFWrapper {
    let d = open_from_path(path.clone().unwrap_or_default().into());
    return DXFWrapper::from(&d);
}
