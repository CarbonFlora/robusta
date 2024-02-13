use crate::*;

pub fn draw_texts(commands: &mut Commands, wrapper: &DXFWrapper) {
    for text in &wrapper.text {
        let text_body = Text::from_section(text.body.clone(), TextStyle::default());
        let origin = text.coordinates.xyz();

        commands.spawn((
            Text2dBundle {
                text: text_body,
                text_anchor: bevy::sprite::Anchor::Center,
                transform: Transform::from_translation(Vec3::new(origin[0], origin[1], origin[2])),
                text_layout_info: bevy::text::TextLayoutInfo::default(),
                ..default()
            },
            PickableBundle::default(),
            On::<Pointer<Select>>::send_event::<SelectionInstance>(),
            On::<Pointer<Deselect>>::send_event::<SelectionInstance>(),
        ));
    }
}
