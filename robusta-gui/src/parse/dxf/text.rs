use super::*;

/// Currently, this is using a sprite to display text. It may be simplier but more resource intensive to use mesh instead.
pub fn spawn_text(
    sp: &dxf::entities::Text,
    co: &mut Commands,
    _me: &mut ResMut<Assets<Mesh>>,
    _ma: &mut ResMut<Assets<ColorMaterial>>,
    ix: &mut TopZLayer,
) {
    let sp = to_rentity(sp);
    let text_body = Text::from_section(sp.body.clone(), TextStyle::default());
    let origin = sp.coordinates.xyz();
    co.spawn((
        Text2dBundle {
            text: text_body,
            text_anchor: bevy::sprite::Anchor::Center,
            transform: Transform::from_translation(Vec3::new(origin[0], origin[1], ix.0 as f32))
                .with_rotation(Quat::from_rotation_z(sp.rotation))
                .with_scale(Vec3::new(sp.height / 5., sp.height / 5., 1.)),
            text_layout_info: bevy::text::TextLayoutInfo::default(),
            ..default()
        },
        REntity::Text(sp),
        PickableBundle::default(),
        On::<Pointer<Select>>::send_event::<Selection>(),
        On::<Pointer<Deselect>>::send_event::<Selection>(),
    ));
}

fn to_rentity(sp: &dxf::entities::Text) -> robusta_core::text::Text {
    let origin = Point::new(sp.location.x as f32, sp.location.y as f32, 0.);

    robusta_core::text::Text {
        coordinates: origin,
        body: sp.value.clone(),
        rotation: sp.rotation as f32,
        height: sp.text_height as f32,
    }
}
