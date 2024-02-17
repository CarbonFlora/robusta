use crate::parse::*;
use crate::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RobustaEntities {
    pub points: Vec<robusta_core::point::Point>,
    pub lines: Vec<robusta_core::line::Line>,
    pub arcs: Vec<robusta_core::arc::Arc>,
    pub circles: Vec<robusta_core::circle::Circle>,
    pub text: Vec<robusta_core::text::Text>,
}

impl RobustaEntities {
    pub fn new() -> Self {
        RobustaEntities::default()
    }

    pub fn from(drawing: &Drawing) -> Self {
        let points = get_points(drawing);
        let (lines, arcs, circles) = get_segments(drawing); //this is garbage way rn
        let text = get_text(drawing);

        RobustaEntities {
            points,
            lines,
            arcs,
            circles,
            text,
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
            EntityType::Text(_) => (),
            EntityType::MText(_) => (),
            _ => core::panic!("Uncaptured entity: {entity:#?} "),
        };
    }
    points
}

fn get_segments(
    drawing: &Drawing,
    // points: &Vec<robusta_core::point::Point>,
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
            EntityType::Text(_) => (),
            EntityType::MText(_) => (),
            _ => core::panic!("Uncaptured entity: {entity:#?} "),
        };
    }
    (lines, arcs, circles)
}

fn get_text(drawing: &Drawing) -> Vec<robusta_core::text::Text> {
    let mut texts = Vec::new();
    for entity in drawing.entities() {
        match &entity.specific {
            EntityType::Text(specific) => texts.push(text::to_text(specific)),
            // EntityType::ArcAlignedText(specific) => texts.extend(),
            EntityType::MText(specific) => texts.push(text::to_text_mtext(specific)),
            // EntityType::RText(specific) => texts.extend(),
            // _ => core::panic!("Uncaptured entity: {entity:#?} "),
            _ => (),
        };
    }
    texts
}
