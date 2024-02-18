use crate::*;

pub fn to_text(specific: &Text) -> robusta_core::RobustaEntity {
    let origin = Point::new(specific.location.x as f32, specific.location.y as f32, 0.);

    robusta_core::RobustaEntity::Text(robusta_core::text::Text {
        coordinates: origin,
        body: specific.value.clone(),
        rotation: specific.rotation as f32,
        height: specific.text_height as f32,
    })
}

pub fn to_text_mtext(specific: &MText) -> robusta_core::text::Text {
    let origin = Point::new(
        specific.insertion_point.x as f32,
        specific.insertion_point.y as f32,
        0.,
    );

    robusta_core::text::Text {
        coordinates: origin,
        body: specific.text.clone(),
        rotation: specific.rotation_angle as f32,
        height: specific.initial_text_height as f32,
    }
}
