// use bevy::render::render_asset::RenderAssetUsages;

// use super::*;

// pub fn spawn_arc(
//     sp: &dxf::entities::Arc,
//     co: &mut Commands,
//     me: &mut ResMut<Assets<Mesh>>,
//     ma: &mut ResMut<Assets<ColorMaterial>>,
//     ix: &mut TopZLayer,
// ) {
//     let lw = 0.3f32;
//     let sp = to_rentity(sp);
//     co.spawn((
//         MaterialMesh2dBundle {
//             mesh: me.add(arc_mesh(lw, &sp)).into(),
//             material: ma.add(ColorMaterial::from(Color::WHITE)),
//             transform: Transform::from_translation(Vec3::new(0., 0., ix.top() as f32)),
//             ..default()
//         },
//         REntity::Arc(sp),
//         PickableBundle::default(),
//         On::<Pointer<Select>>::send_event::<Selection>(),
//         On::<Pointer<Deselect>>::send_event::<Selection>(),
//     ));
// }

// fn to_rentity(sp: &dxf::entities::Arc) -> robusta_core::arc::Arc {
//     let x1 = sp.center.x + sp.start_angle.to_radians().cos() * sp.radius;
//     let y1 = sp.center.y + sp.start_angle.to_radians().sin() * sp.radius;
//     let point1 = Point::new(x1 as f32, y1 as f32, 0.);

//     let x2 = sp.center.x + sp.end_angle.to_radians().cos() * sp.radius;
//     let y2 = sp.center.y + sp.end_angle.to_radians().sin() * sp.radius;
//     let point2 = Point::new(x2 as f32, y2 as f32, 0.);

//     let mut p3_angle_rad = ((sp.start_angle + sp.end_angle) / 2.).to_radians() as f32;
//     if sp.start_angle > sp.end_angle {
//         p3_angle_rad -= PI;
//     }

//     let (p3_x, p3_y) = (
//         sp.center.x as f32 + sp.radius as f32 * p3_angle_rad.cos(),
//         sp.center.y as f32 + sp.radius as f32 * p3_angle_rad.sin(),
//     );
//     let lazy_point = Point::new(p3_x, p3_y, 0.);

//     robusta_core::arc::Arc::new([point1, point2, lazy_point])
// }
