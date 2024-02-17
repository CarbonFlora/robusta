use crate::*;

pub fn to_text(specific: &Text) -> robusta_core::text::Text {
    let origin = Point::new(specific.location.x as f32, specific.location.y as f32, 0.);

    robusta_core::text::Text {
        coordinates: origin,
        body: specific.value.clone(),
    }
}
