use crate::parse::*;
use crate::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DXFWrapper {
    pub points: Vec<Point>,
}

impl DXFWrapper {
    pub fn new() -> Self {
        return DXFWrapper::default();
    }

    pub fn from(drawing: &Drawing) -> Self {
        DXFWrapper {
            points: get_points(drawing),
        }
    }
}

fn get_points(drawing: &Drawing) -> Vec<Point> {
    let mut points = Vec::new();
    for entity in drawing.entities() {
        match &entity.specific {
            EntityType::Line(specific) => points.extend(line::to_points(specific)),
            EntityType::Arc(specific) => points.extend(arc::to_points(specific)),
            EntityType::LwPolyline(specific) => points.extend(lwpolyline::to_points(specific)),
            EntityType::Circle(specific) => points.extend(circle::to_points(specific)),
            _ => core::panic!("Uncaptured entity: {entity:#?} "),
        };
    }
    return points;
}
