use crate::*;

/// This actually draws from ViewportState. Gizmos are inherently not pickable.
/// For bundle: 'PickableBundle::default(),' to be implimented, commands.spawn must be used.
pub fn draw_first(
    // mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    // viewport_state: Res<ViewportState>,
    ui_state: Res<UiState>,
    mut gizmos: Gizmos,
) {
    for file in &ui_state.loaded_files {
        for point in &file.1.points {
            // println!("point drawn: {:?}", point);
            gizmos.circle_2d(
                Vec2::new(point.coordinates.x, point.coordinates.y),
                1.,
                Color::ALICE_BLUE,
            );
        }
    }
    //     // commands.spawn((
    //     //     PbrBundle {
    //     //         mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     //         material: materials.add(Color::WHITE.into()),
    //     //         transform: Transform::from_xyz(point.coordinates.x, point.coordinates.y, 0.0),
    //     //         ..default()
    //     //     },
    //     // PickableBundle::default(),
    //     // ));
    // }
}
