use crate::*;

pub fn draw_arcs(
    entity_package: &mut (
        &mut Commands,
        &mut ResMut<Assets<Mesh>>,
        &mut ResMut<Assets<ColorMaterial>>,
    ),
    wrapper: &RobustaEntities,
    entity_mapping: &mut EntityMapping,
) {
    let line_width = 0.3f32;
    for arc in &wrapper.arcs {
        let id = entity_package
            .0
            .spawn((
                MaterialMesh2dBundle {
                    mesh: entity_package.1.add(arc_mesh(line_width, arc)).into(),
                    material: entity_package.2.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_translation(Vec3::new(0., 0., 7.)),
                    ..default()
                },
                PickableBundle::default(),
                On::<Pointer<Select>>::send_event::<SelectionInstance>(),
                On::<Pointer<Deselect>>::send_event::<SelectionInstance>(),
            ))
            .id();
        entity_mapping
            .hash
            .insert(id, robusta_core::RobustaEntity::Arc(arc.clone()));
    }
}

fn arc_mesh(line_width: f32, arc: &robusta_core::arc::Arc) -> Mesh {
    let lw_half = line_width / 2.0f32;
    let num_segments = 30u32;
    let vertexes: Vec<[f32; 3]> = arc_vertexes(num_segments, arc, lw_half);
    let triangle_indexes: Vec<u32> = arc_indexes(num_segments);

    Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 0., 1.]; vertexes.len()])
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertexes)
        .with_indices(Some(Indices::U32(triangle_indexes)))
}

fn arc_vertexes(num_segments: u32, arc: &robusta_core::arc::Arc, lw_half: f32) -> Vec<[f32; 3]> {
    let mut vertexes = Vec::new();
    let spec = arc.specifications();
    let angle_increment = spec.angle / num_segments as f32;

    for i in 0..=num_segments {
        let angle_offset = spec.start_angle_rad + angle_increment * i as f32;

        let x_outer = spec.center.coordinates.x + (spec.radius + lw_half) * (angle_offset).cos();
        let y_outer = spec.center.coordinates.y + (spec.radius + lw_half) * (angle_offset).sin();
        let x_inner = spec.center.coordinates.x + (spec.radius - lw_half) * (angle_offset).cos();
        let y_inner = spec.center.coordinates.y + (spec.radius - lw_half) * (angle_offset).sin();

        vertexes.push([x_outer, y_outer, 0.]);
        vertexes.push([x_inner, y_inner, 0.]);
    }

    vertexes
}

fn arc_indexes(num_segments: u32) -> Vec<u32> {
    let mut a = Vec::new();

    for i in 0..(num_segments * 2) {
        a.extend(vec![i, i + 1, i + 2]);
    }

    a
}
