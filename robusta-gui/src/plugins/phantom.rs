use crate::REntity;

use self::{
    construction::ConstructionInput,
    snap::{SnapPoint, SnapTo},
};

use super::*;

pub struct PhantomPlugin;
impl bevy::app::Plugin for PhantomPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(PhantomSnaps::new())
            .add_systems(PreUpdate, update_phantom_snap)
            .add_systems(Update, update_rphantom_pointer);
    }
}

/// If this is Some(), then the phantom point coordiates will automatically be snapped to the hovered snap.
#[derive(Debug, Resource, Default)]
pub struct PhantomSnaps {
    snap_to: Option<Vec2>,
}

impl PhantomSnaps {
    pub fn new() -> Self {
        PhantomSnaps::default()
    }

    pub fn reset(&mut self) {
        self.snap_to = None;
    }
}

/// This is a marker component to delineate a point entity in the process of being placed.
#[derive(Debug, Component, Default)]
pub struct RPhantomPointer;

pub fn despawn_all_phantoms(
    c: &mut Commands,
    ewp: &Query<Entity, With<RPhantomPointer>>,
    fs: &mut ResMut<PhantomSnaps>,
) {
    for e in ewp.iter() {
        c.entity(e).remove::<RPhantomPointer>();
        c.entity(e).despawn();
    }
    fs.snap_to = None;
}

pub fn update_phantom_snap(
    mut pr: ResMut<PhantomSnaps>,
    mut ers: EventReader<SnapTo>,
    resp: Query<&REntity, (With<SnapPoint>, Without<RPhantomPointer>)>,
) {
    for s in ers.read() {
        match s.1 {
            true => {
                let snap_point_xyz = resp.get(s.0).unwrap().unwrap_point().coordinates;
                pr.snap_to = Some(Vec2::new(snap_point_xyz.x, snap_point_xyz.y));
            }
            false => {
                pr.snap_to = None;
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn update_rphantom_pointer(
    pr: Res<PhantomSnaps>,
    // preg: Res<PhantomREntityGeo>,
    mut ewp: Query<
        (&mut Transform, &mut REntity),
        (With<RPhantomPointer>, Without<bevy_pancam::PanCam>),
    >,
    w: Query<&Window, With<PrimaryWindow>>,
    transform: Query<(&Camera, &GlobalTransform), With<bevy_pancam::PanCam>>,
) {
    let (ca, gt) = transform.single();
    // if let Ok((mut tr, re)) = ewp.get_single_mut() {
    //     let sp = re.into_inner().unwrap_point_mut();
    //     if let Some(cursor_world_pos) = w
    //         .single()
    //         .cursor_position()
    //         .and_then(|cursor_pos| ca.viewport_to_world_2d(gt, cursor_pos))
    //     {
    //         let binding_xy = match pr.snap_to {
    //             Some(sv) => sv,
    //             None => cursor_world_pos,
    //         };

    //         sp.xy_mut(binding_xy.x, binding_xy.y);
    //         tr.translation.x = binding_xy.x;
    //         tr.translation.y = binding_xy.y;
    //     }
    // }
    for (mut tr, mut re) in ewp.iter_mut() {
        let sp = re.unwrap_point_mut();
        if let Some(cursor_world_pos) = w
            .single()
            .cursor_position()
            .and_then(|cursor_pos| ca.viewport_to_world_2d(gt, cursor_pos))
        {
            let binding_xy = match pr.snap_to {
                Some(sv) => sv,
                None => cursor_world_pos,
            };

            sp.xy_mut(binding_xy.x, binding_xy.y);
            tr.translation.x = binding_xy.x;
            tr.translation.y = binding_xy.y;
        }
    }
}

pub fn index_point(
    qre: &Query<&REntity, (With<RPhantomPointer>, Without<bevy_pancam::PanCam>)>,
    ewci: &mut EventWriter<ConstructionInput>,
) {
    let re = match qre.get_single() {
        Ok(w) => w,
        Err(_) => return,
    };
    let xyz = re.unwrap_point().coordinates;
    let coords = Vec3::new(xyz.x, xyz.y, xyz.z);
    ewci.send(ConstructionInput { coords });
}

// #[allow(clippy::type_complexity)]
// pub fn update_rphantom(
//     pr: Res<PhantomSnaps>,
//     preg: Res<PhantomREntityGeo>,
//     mut ewp: Query<(&mut Transform, &mut REntity), (With<RPhantom>, Without<bevy_pancam::PanCam>)>,
//     w: Query<&Window, With<PrimaryWindow>>,
//     transform: Query<(&Camera, &GlobalTransform), With<bevy_pancam::PanCam>>,
// ) {
//     let (ca, gt) = transform.single();
//     for (tr, re) in ewp.iter_mut() {
//         let tr = tr.into_inner();
//         let re = re.into_inner();
//         match re {
//             REntity::Arc(sp) => update_rphantom_definition(&mut sp.definition, &preg),
//             REntity::Circle(sp) => update_rphantom_definition(&mut sp.definition, &preg),
//             REntity::Line(sp) => update_rphantom_definition(&mut sp.definition, &preg),
//             REntity::Point(sp) => update_rphantom_pointer(&w, ca, gt, &pr, sp, tr),
//             REntity::Text(sp) => update_rphantom_definition(&mut sp.definition, &preg),
//         }
//     }
// }

// pub fn update_rphantom_mesh(
//     mut qmcre: Query<(&mut Mesh2dHandle, &REntity), With<RPhantom>>,
//     mut me: ResMut<Assets<Mesh>>,
// ) {
//     let lw = 0.3f32;

//     for (m, re) in qmcre.iter_mut() {
//         match re {
//             REntity::Arc(_) => todo!(),
//             REntity::Circle(_) => todo!(),
//             REntity::Line(sp) => {
//                 let spec = sp.specifications();
//                 me.insert(&m.0, line_mesh(lw, spec.length, spec.h_angle));
//             }
//             REntity::Point(_) => (),
//             REntity::Text(_) => todo!(),
//         }
//     }
// }

// fn update_rphantom_definition(sp: &mut [Point], preg: &Res<PhantomREntityGeo>) {
//     let mut a = preg.definition.iter();
//     for p in sp {
//         if let Some(c) = a.next() {
//             p.xy_mut(c.x, c.y);
//         }
//     }
// }

// pub fn canonize(
//     c: &mut Commands,
//     ewp: &Query<Entity, With<RPhantom>>,
//     ewrsp: &mut EventWriter<UpdateSnapPoints>,
// ) {
//     ewrsp.send(UpdateSnapPoints(false));
//     for e in ewp.iter() {
//         normalize(c, e);
//     }
// }

// fn normalize(c: &mut Commands, e: Entity) {
//     c.entity(e).insert((
//         PickableBundle::default(),
//         On::<Pointer<Select>>::send_event::<Selection>(),
//         On::<Pointer<Deselect>>::send_event::<Selection>(),
//     ));
//     c.entity(e).remove::<RPhantom>();
// }

// pub fn spawn_phantom_line(
//     co: &mut Commands,
//     me: &mut ResMut<Assets<Mesh>>,
//     ma: &mut ResMut<Assets<ColorMaterial>>,
//     tzi: &mut TopZLayer,
//     ewrsp: &mut EventWriter<UpdateSnapPoints>,
// ) {
//     ewrsp.send(UpdateSnapPoints(true));
//     let sp = Line::new([Point::new(0., 0., 0.), Point::new(1., 0., 0.)]);
//     let id = spawn_line_mesh(sp, co, me, ma, tzi);
//     co.entity(id).insert(RPhantom);
// }
