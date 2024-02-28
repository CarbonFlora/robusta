use robusta_core::point;

use self::line::line_mesh;

use super::*;

pub fn spawn_lwpolyline(
    sp: &dxf::entities::LwPolyline,
    co: &mut Commands,
    me: &mut ResMut<Assets<Mesh>>,
    ma: &mut ResMut<Assets<ColorMaterial>>,
    ix: &mut TopZLayer,
) {
    let lw = 0.3f32;
    let spv = to_rentity(sp);
    for sp in spv {
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
            PickableBundle::default(),
            On::<Pointer<Select>>::send_event::<Selection>(),
            On::<Pointer<Deselect>>::send_event::<Selection>(),
        ));
    }
}

fn to_rentity(sp: &dxf::entities::LwPolyline) -> Vec<robusta_core::line::Line> {
    let mut lv = Vec::new();
    let mut spviter = sp.vertices.iter();
    let mut pre_v = spviter.next().unwrap();
    for current_v in spviter {
        lv.push(robusta_core::line::Line {
            definition: [
                point::Point::new(pre_v.x as f32, pre_v.y as f32, 0.),
                point::Point::new(current_v.x as f32, current_v.y as f32, 0.),
            ],
        });

        pre_v = current_v;
    }

    lv
}
