use crate::*;

pub fn draw_texts(
    entity_package: &mut (
        &mut Commands,
        &mut ResMut<Assets<Mesh>>,
        &mut ResMut<Assets<ColorMaterial>>,
    ),
    wrapper: &RobustaEntities,
    entity_mapping: &mut EntityMapping,
) {
    for text in &wrapper.text {
        let text_body = Text::from_section(text.body.clone(), TextStyle::default());
        let origin = text.coordinates.xyz();

        let id = entity_package
            .0
            .spawn((
                Text2dBundle {
                    text: text_body,
                    text_anchor: bevy::sprite::Anchor::Center,
                    transform: Transform::from_translation(Vec3::new(
                        origin[0], origin[1], origin[2],
                    ))
                    .with_rotation(Quat::from_rotation_z(text.rotation))
                    .with_scale(Vec3::new(
                        text.height / 5.,
                        text.height / 5.,
                        1.,
                    )),
                    text_layout_info: bevy::text::TextLayoutInfo::default(),
                    ..default()
                },
                PickableBundle::default(),
                On::<Pointer<Select>>::send_event::<SelectionInstance>(),
                On::<Pointer<Deselect>>::send_event::<SelectionInstance>(),
            ))
            .id();
        entity_mapping
            .hash
            .insert(id, robusta_core::RobustaEntity::Text(text.clone()));
    }
}
