use crate::*;

pub fn draw_lines(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    wrapper: &DXFWrapper,
) {
    let line_width = 0.3f32;
    for line in &wrapper.lines {
        let spec = line.specifications();

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(line_mesh(line_width, spec.length, spec.h_angle).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(Vec3::new(
                    line.definition[0].coordinates.x,
                    line.definition[0].coordinates.y,
                    8.,
                )),
                ..default()
            },
            PickableBundle::default(),
            On::<Pointer<Select>>::send_event::<SelectionInstance>(),
            On::<Pointer<Deselect>>::send_event::<SelectionInstance>(),
        ));
    }
}

fn line_mesh(line_width: f32, length: f32, angle_rad: f32) -> Mesh {
    let lw_half = line_width / 2.0f32;
    Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                [-lw_half * angle_rad.sin(), lw_half * angle_rad.cos(), 0.0],
                [lw_half * angle_rad.sin(), -lw_half * angle_rad.cos(), 0.0],
                [
                    length * angle_rad.cos() + lw_half * angle_rad.sin(),
                    length * angle_rad.sin() - lw_half * angle_rad.cos(),
                    0.0,
                ],
                [
                    length * angle_rad.cos() - lw_half * angle_rad.sin(),
                    length * angle_rad.sin() + lw_half * angle_rad.cos(),
                    0.0,
                ],
            ],
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            vec![
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
            ],
        )
        .with_indices(Some(Indices::U32(vec![0, 3, 1, 1, 3, 2])))
}
