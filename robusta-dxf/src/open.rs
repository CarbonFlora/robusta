use std::path::PathBuf;

use crate::*;

pub fn open(path: PathBuf) -> Result<()> {
    let drawing = Drawing::load_file(path)?;
    for e in drawing.entities() {
        println!("found {:?} on layer {}", e, e.common.layer);
        match e.specific {
            EntityType::Circle(ref _circle) => {
                // do something with the circle
            }
            EntityType::Line(ref _line) => {
                // do something with the line
            }
            _ => (),
        };
    }
    Ok(())
}
