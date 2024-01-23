use crate::*;

/// This actually draws from ViewportState. Gizmos are inherently not pickable.
/// For bundle: 'PickableBundle::default(),' to be implimented, commands.spawn must be used.
pub fn draw_dxf(
    // mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    viewport_state: Res<ViewportState>,
    mut gizmos: Gizmos,
) {
    for point in &viewport_state.points {
        // println!("point drawn: {:?}", point);
        gizmos.circle_2d(
            Vec2::new(point.coordinates.x, point.coordinates.y),
            1.,
            Color::ALICE_BLUE,
        );

        // commands.spawn((
        //     PbrBundle {
        //         mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        //         material: materials.add(Color::WHITE.into()),
        //         transform: Transform::from_xyz(point.coordinates.x, point.coordinates.y, 0.0),
        //         ..default()
        //     },
        // PickableBundle::default(),
        // ));
    }
}
