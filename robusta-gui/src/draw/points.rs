use crate::*;

pub fn draw_points(
    entity_package: &mut (
        &mut Commands,
        &mut ResMut<Assets<Mesh>>,
        &mut ResMut<Assets<ColorMaterial>>,
    ),
    wrapper: &RobustaEntities,
    entity_mapping: &mut EntityMapping,
) {
    for point in &wrapper.points {
        let id = entity_package
            .0
            .spawn((
                MaterialMesh2dBundle {
                    mesh: entity_package.1.add(shape::Circle::new(0.5).into()).into(),
                    material: entity_package.2.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_translation(Vec3::new(
                        point.coordinates.x,
                        point.coordinates.y,
                        9.,
                    )),
                    ..default()
                },
                PickableBundle::default(),
                On::<Pointer<Select>>::send_event::<SelectionInstance>(),
                On::<Pointer<Deselect>>::send_event::<SelectionInstance>(),
            ))
            .id();
        entity_mapping
            .hash
            .insert(id, robusta_core::RobustaEntity::Point(point.clone()));
    }
}
