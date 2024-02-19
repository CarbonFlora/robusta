use robusta_core::circle::Circle;

use crate::*;

pub fn draw_circles(
    entity_package: &mut (
        &mut Commands,
        &mut ResMut<Assets<Mesh>>,
        &mut ResMut<Assets<ColorMaterial>>,
    ),
    specific: &Circle,
    entity_mapping: &mut EntityMapping,
    index: usize,
) {
    let line_width = 0.3f32;
    let id = entity_package
        .0
        .spawn((
            MaterialMesh2dBundle {
                mesh: entity_package
                    .1
                    .add(circle_mesh(line_width, specific))
                    .into(),
                material: entity_package.2.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(Vec3::new(0., 0., index as f32)),
                ..default()
            },
            PickableBundle::default(),
            On::<Pointer<Select>>::send_event::<SelectionInstance>(),
            On::<Pointer<Deselect>>::send_event::<SelectionInstance>(),
        ))
        .id();
    entity_mapping
        .hash
        .insert(id, robusta_core::RobustaEntity::Circle(specific.clone()));
}

fn circle_mesh(line_width: f32, circle: &robusta_core::circle::Circle) -> Mesh {
    let lw_half = line_width / 2.0f32;
    let num_segments = 30u32;
    let vertexes: Vec<[f32; 3]> = circle_vertexes(num_segments, circle, lw_half);
    let triangle_indexes: Vec<u32> = arc_indexes(num_segments);

    Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 0., 1.]; vertexes.len()])
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertexes)
        .with_indices(Some(Indices::U32(triangle_indexes)))
}

fn arc_indexes(num_segments: u32) -> Vec<u32> {
    let mut a = Vec::new();

    for i in 0..(num_segments * 2) {
        a.extend(vec![i, i + 1, i + 2]);
    }

    a
}

fn circle_vertexes(
    num_segments: u32,
    circle: &robusta_core::circle::Circle,
    lw_half: f32,
) -> Vec<[f32; 3]> {
    let mut vertexes = Vec::new();
    let spec = circle.specifications();
    let angle_increment = (2. * PI) / num_segments as f32;

    for i in 0..=num_segments {
        let angle_offset = angle_increment * i as f32;

        let x_outer =
            circle.definition[1].coordinates.x + (spec.radius + lw_half) * (angle_offset).cos();
        let y_outer =
            circle.definition[1].coordinates.y + (spec.radius + lw_half) * (angle_offset).sin();
        let x_inner =
            circle.definition[1].coordinates.x + (spec.radius - lw_half) * (angle_offset).cos();
        let y_inner =
            circle.definition[1].coordinates.y + (spec.radius - lw_half) * (angle_offset).sin();

        vertexes.push([x_outer, y_outer, 0.]);
        vertexes.push([x_inner, y_inner, 0.]);
    }

    vertexes
}
