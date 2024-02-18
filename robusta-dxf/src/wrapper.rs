use robusta_core::RobustaEntity;

use crate::parse::*;
use crate::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RFile {
    pub entities: Vec<RobustaEntity>,
}

impl RFile {
    pub fn new() -> Self {
        RFile::default()
    }

    pub fn from(drawing: &Drawing) -> Self {
        let mut rentities = RFile::new();

        for entity in drawing.entities() {
            match &entity.specific {
                EntityType::Line(specific) => rentities.entities.push(line::to_segment(specific)),
                EntityType::Arc(specific) => rentities.entities.push(arc::to_segment(specific)),
                EntityType::LwPolyline(specific) => {
                    rentities.entities.extend(lwpolyline::to_segments(specific))
                }
                EntityType::Circle(specific) => {
                    rentities.entities.push(circle::to_segment(specific))
                }
                EntityType::Text(specific) => rentities.entities.push(text::to_text(specific)),
                // EntityType::ArcAlignedText(specific) => texts.extend(),
                // EntityType::MText(specific) => rentities.text.push(text::to_text_mtext(specific)),
                _ => core::panic!("Uncaptured entity: {entity:#?} "),
            };
        }

        rentities
    }

    pub fn iter_points(&self) -> Vec<&Point> {
        let mut points = Vec::new();
        for i in &self.entities {
            match i {
                RobustaEntity::Point(specific) => points.push(specific),
                RobustaEntity::Arc(specific) => points.extend(specific.definition.iter()),
                RobustaEntity::Circle(specific) => points.extend(specific.definition.iter()),
                RobustaEntity::Line(specific) => points.extend(specific.definition.iter()),
                RobustaEntity::Text(specific) => points.push(&specific.coordinates),
            }
        }
        points
    }
}
