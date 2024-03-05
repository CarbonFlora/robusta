use super::*;

pub fn spawn_line(
    sp: &dxf::entities::Line,
    co: &mut Commands,
    me: &mut ResMut<Assets<Mesh>>,
    ma: &mut ResMut<Assets<ColorMaterial>>,
    ix: &mut TopZLayer,
) {
    let sp = to_rentity(sp);
    let id = spawn_line_mesh(sp, co, me, ma, ix);
    co.entity(id).insert((
        PickableBundle::default(),
        On::<Pointer<Select>>::send_event::<Selection>(),
        On::<Pointer<Deselect>>::send_event::<Selection>(),
    ));
}

pub fn spawn_line_mesh(
    sp: robusta_core::line::Line,
    co: &mut Commands,
    me: &mut ResMut<Assets<Mesh>>,
    ma: &mut ResMut<Assets<ColorMaterial>>,
    ix: &mut TopZLayer,
) -> Entity {
    let lw = 0.3f32;
    let spec = sp.specifications();
    co.spawn((
        MaterialMesh2dBundle {
            mesh: me.add(line_mesh(lw, spec.length, spec.h_angle)).into(),
            material: ma.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(
                sp.definition[0].coordinates.x,
                sp.definition[0].coordinates.y,
                ix.top() as f32,
            )),
            ..default()
        },
        REntity::Line(sp),
    ))
    .id()
}

fn to_rentity(sp: &dxf::entities::Line) -> robusta_core::line::Line {
    let point1 = Point::new(sp.p1.x as f32, sp.p1.y as f32, 0.);
    let point2 = Point::new(sp.p2.x as f32, sp.p2.y as f32, 0.);
    robusta_core::line::Line::new([point1, point2])
}

pub fn line_mesh(line_width: f32, length: f32, angle_rad: f32) -> Mesh {
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
