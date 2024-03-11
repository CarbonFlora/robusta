// use crate::{parse::bevy::line::spawn_line_mesh, plugins::selection::Selection};

// use super::*;

// pub fn spawn_line(
//     sp: &dxf::entities::Line,
//     co: &mut Commands,
//     me: &mut ResMut<Assets<Mesh>>,
//     ma: &mut ResMut<Assets<ColorMaterial>>,
//     ix: &mut TopZLayer,
// ) {
//     let sp = to_rentity(sp);
//     let id = spawn_line_mesh(sp, co, me, ma, ix);
//     co.entity(id).insert((
//         PickableBundle::default(),
//         On::<Pointer<Select>>::send_event::<Selection>(),
//         On::<Pointer<Deselect>>::send_event::<Selection>(),
//     ));
// }

// pub fn spawn_line_mesh(
//     sp: robusta_core::line::Line,
//     co: &mut Commands,
//     me: &mut ResMut<Assets<Mesh>>,
//     ma: &mut ResMut<Assets<ColorMaterial>>,
//     tz: &mut TopZLayer,
// ) -> Entity {
//     let lw = 0.3f32;
//     let spec = sp.specifications();
//     co.spawn((
//         MaterialMesh2dBundle {
//             mesh: me.add(line_mesh(lw, spec.length, spec.h_angle)).into(),
//             material: ma.add(ColorMaterial::from(Color::WHITE)),
//             transform: Transform::from_translation(Vec3::new(
//                 sp.definition[0].coordinates.x,
//                 sp.definition[0].coordinates.y,
//                 tz.top() as f32,
//             )),
//             ..default()
//         },
//         REntity::Line(sp),
//     ))
//     .id()
// }
