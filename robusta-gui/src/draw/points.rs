use robusta_core::point::Point;

use crate::*;

pub fn draw_points(
    entity_package: &mut (
        &mut Commands,
        &mut ResMut<Assets<Mesh>>,
        &mut ResMut<Assets<ColorMaterial>>,
    ),
    specific: &Point,
    entity_mapping: &mut EntityMapping,
    index: usize,
) {
    let id = entity_package
        .0
        .spawn((
            MaterialMesh2dBundle {
                mesh: entity_package.1.add(shape::Circle::new(0.5).into()).into(),
                material: entity_package.2.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(Vec3::new(
                    specific.coordinates.x,
                    specific.coordinates.y,
                    index as f32,
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
        .insert(id, robusta_core::RobustaEntity::Point(specific.clone()));
}
