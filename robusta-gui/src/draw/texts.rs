use crate::*;

use self::rselection::Selection;

pub fn draw_texts(
    entity_package: &mut (
        &mut Commands,
        &mut ResMut<Assets<Mesh>>,
        &mut ResMut<Assets<ColorMaterial>>,
    ),
    specific: &robusta_core::text::Text,
    entity_mapping: &mut EntityMapping,
    index: usize,
) {
    let text_body = Text::from_section(specific.body.clone(), TextStyle::default());
    let origin = specific.coordinates.xyz();

    let id = entity_package
        .0
        .spawn((
            Text2dBundle {
                text: text_body,
                text_anchor: bevy::sprite::Anchor::Center,
                transform: Transform::from_translation(Vec3::new(
                    origin[0],
                    origin[1],
                    index as f32,
                ))
                .with_rotation(Quat::from_rotation_z(specific.rotation))
                .with_scale(Vec3::new(
                    specific.height / 5.,
                    specific.height / 5.,
                    1.,
                )),
                text_layout_info: bevy::text::TextLayoutInfo::default(),
                ..default()
            },
            PickableBundle::default(),
            On::<Pointer<Select>>::send_event::<Selection>(),
            On::<Pointer<Deselect>>::send_event::<Selection>(),
        ))
        .id();
    entity_mapping
        .hash
        .insert(id, robusta_core::RobustaEntity::Text(specific.clone()));
}
