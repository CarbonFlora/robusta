use super::*;

pub fn spawn_arc(
    sp: &dxf::entities::Arc,
    co: &mut Commands,
    me: &mut ResMut<Assets<Mesh>>,
    ma: &mut ResMut<Assets<ColorMaterial>>,
    ix: usize,
) {
    let lw = 0.3f32;
    let sp = to_rentity(sp);
    co.spawn((
        MaterialMesh2dBundle {
            mesh: me.add(arc_mesh(lw, &sp)).into(),
            material: ma.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0., 0., ix as f32)),
            ..default()
        },
        REntity::Arc(sp),
        PickableBundle::default(),
        On::<Pointer<Select>>::send_event::<Selection>(),
        On::<Pointer<Deselect>>::send_event::<Selection>(),
    ));
}

fn to_rentity(sp: &dxf::entities::Arc) -> robusta_core::arc::Arc {
    let x1 = sp.center.x + sp.start_angle.to_radians().cos() * sp.radius;
    let y1 = sp.center.y + sp.start_angle.to_radians().sin() * sp.radius;
    let point1 = Point::new(x1 as f32, y1 as f32, 0.);

    let x2 = sp.center.x + sp.end_angle.to_radians().cos() * sp.radius;
    let y2 = sp.center.y + sp.end_angle.to_radians().sin() * sp.radius;
    let point2 = Point::new(x2 as f32, y2 as f32, 0.);

    let mut p3_angle_rad = ((sp.start_angle + sp.end_angle) / 2.).to_radians() as f32;
    if sp.start_angle > sp.end_angle {
        p3_angle_rad -= PI;
    }

    let (p3_x, p3_y) = (
        sp.center.x as f32 + sp.radius as f32 * p3_angle_rad.cos(),
        sp.center.y as f32 + sp.radius as f32 * p3_angle_rad.sin(),
    );
    let lazy_point = Point::new(p3_x, p3_y, 0.);

    robusta_core::arc::Arc::new([point1, point2, lazy_point])
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
