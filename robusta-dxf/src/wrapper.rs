use crate::parse::*;
use crate::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DXFWrapper {
    pub points: Vec<robusta_core::point::Point>,
    pub lines: Vec<robusta_core::line::Line>,
    pub arcs: Vec<robusta_core::arc::Arc>,
    pub circles: Vec<robusta_core::circle::Circle>,
}

impl DXFWrapper {
    pub fn new() -> Self {
        return DXFWrapper::default();
    }

    pub fn from(drawing: &Drawing) -> Self {
        let points = get_points(drawing);
        let (lines, arcs, circles) = get_segments(drawing); //this is garbage way rn

        DXFWrapper {
            points,
            lines,
            arcs,
            circles,
        }
    }
}

fn get_points(drawing: &Drawing) -> Vec<robusta_core::point::Point> {
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

fn get_segments(
    drawing: &Drawing,
) -> (
    Vec<robusta_core::line::Line>,
    Vec<robusta_core::arc::Arc>,
    Vec<robusta_core::circle::Circle>,
) {
    let (mut lines, mut arcs, mut circles) = (Vec::new(), Vec::new(), Vec::new());
    for entity in drawing.entities() {
        match &entity.specific {
            EntityType::Line(specific) => lines.push(line::to_segment(specific)),
            EntityType::Arc(specific) => arcs.push(arc::to_segment(specific)),
            EntityType::LwPolyline(specific) => lines.extend(lwpolyline::to_segments(specific)),
            EntityType::Circle(specific) => circles.push(circle::to_segment(specific)),
            _ => core::panic!("Uncaptured entity: {entity:#?} "),
        };
    }
    return (lines, arcs, circles);
}
