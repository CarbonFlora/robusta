// use std::io::Write;

use crate::*;

pub fn open_from_path(path: PathBuf) -> Result<Drawing> {
    let drawing = Drawing::load_file(path)?;

    // todo!() As I currently do not know how to do conditional debug compilation, uncomment when you want to see what's in the file at this point.
    // let mut w = Vec::new();
    // for entity in drawing.entities() {
    //     writeln!(&mut w, "Found: {entity:?}")?;
    // }
    return Ok(drawing);
}
