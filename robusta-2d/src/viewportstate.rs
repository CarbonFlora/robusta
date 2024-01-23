use crate::*;

/// This is the `Bevy` resource containing all the custom GUI elements.
#[derive(Resource)]
pub struct ViewportState {
    pub opened_file_path: Option<String>,
    pub points: Vec<Point>,
}

impl ViewportState {
    pub fn new(path: Option<String>) -> Self {
        let a = parse_dxf(&path);
        ViewportState {
            opened_file_path: path,
            points: a.points,
        }
    }
}

fn parse_dxf(path: &Option<String>) -> DXFWrapper {
    let d = open_from_path(path.clone().unwrap_or_default().into());
    DXFWrapper::from(&d)
}
