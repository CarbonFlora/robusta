use crate::parse::*;
use crate::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DXFWrapper {
    pub primitives: Vec<Point>,
}

impl DXFWrapper {
    fn into_points(&self, drawing: Drawing) -> Vec<Point> {
        let mut points = Vec::new();
        for entity in drawing.entities() {
            match &entity.specific {
                EntityType::Line(specific) => points.extend(line::to_points(specific)),
                EntityType::Arc(specific) => points.extend(arc::to_points(specific)),
                _ => core::panic!("Uncaptured entity: {entity:#?} "),
            };
        }
        return points;
    }
}
