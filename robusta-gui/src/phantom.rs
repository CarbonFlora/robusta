use bevy::sprite::Mesh2dHandle;

use crate::{
    parse::dxf::line::spawn_line_mesh,
    rselection::Selection,
    snap::{Snap, SnapPoint, UpdateSnapPoints},
    REntity, TopZLayer,
};

use self::parse::dxf::line::line_mesh;

use super::*;

pub struct PhantomPlugin;
impl bevy::app::Plugin for PhantomPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(PhantomSnaps::new())
            .insert_resource(PhantomREntityGeo::new())
            .add_systems(PreUpdate, update_phantom_snap)
            .add_systems(Update, update_rphantom)
            .add_systems(Update, update_rphantom_mesh);
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
}

/// This is used to store coordinate data for building REnitities.
#[derive(Debug, Resource, Default)]
pub struct PhantomREntityGeo {
    definition: Vec<Vec2>,
}

impl PhantomREntityGeo {
    pub fn new() -> Self {
        PhantomREntityGeo::default()
    }
}

/// This is a marker component to delineate a point entity in the process of being placed.
#[derive(Debug, Component)]
pub struct RPhantom;

pub fn despawn_all_phantoms(c: &mut Commands, ewp: &Query<Entity, With<RPhantom>>) {
    for e in ewp.iter() {
        c.entity(e).despawn();
    }
}

pub fn canonize(
    c: &mut Commands,
    ewp: &Query<Entity, With<RPhantom>>,
    ewrsp: &mut EventWriter<UpdateSnapPoints>,
) {
    ewrsp.send(UpdateSnapPoints(false));
    for e in ewp.iter() {
        normalize(c, e);
    }
}

fn normalize(c: &mut Commands, e: Entity) {
    c.entity(e).insert((
        PickableBundle::default(),
        On::<Pointer<Select>>::send_event::<Selection>(),
        On::<Pointer<Deselect>>::send_event::<Selection>(),
    ));
    c.entity(e).remove::<RPhantom>();
}

pub fn spawn_phantom_point(
    co: &mut Commands,
    me: &mut ResMut<Assets<Mesh>>,
    ma: &mut ResMut<Assets<ColorMaterial>>,
    tzi: &mut TopZLayer,
    ewrsp: &mut EventWriter<UpdateSnapPoints>,
) {
    ewrsp.send(UpdateSnapPoints(true));
    co.spawn((
        MaterialMesh2dBundle {
            mesh: me.add(bevy::math::primitives::Circle::new(0.5)).into(),
            material: ma.add(ColorMaterial::from(Color::CYAN)),
            transform: Transform::from_translation(Vec3::new(0., 0., tzi.top() as f32)),
            ..default()
        },
        REntity::Point(point::Point::new(0., 0., 0.)),
        RPhantom,
    ));
}

// One function I want is to be able to create multiple start points and have all the phantom non-points to update to the phantom point.
pub fn spawn_phantom_line(
    co: &mut Commands,
    me: &mut ResMut<Assets<Mesh>>,
    ma: &mut ResMut<Assets<ColorMaterial>>,
    tzi: &mut TopZLayer,
    ewrsp: &mut EventWriter<UpdateSnapPoints>,
) {
    ewrsp.send(UpdateSnapPoints(true));
    let sp = Line::new([Point::new(0., 0., 0.), Point::new(1., 0., 0.)]);
    let id = spawn_line_mesh(sp, co, me, ma, tzi);
    co.entity(id).insert(RPhantom);
}

pub fn update_phantom_snap(
    mut pr: ResMut<PhantomSnaps>,
    mut ers: EventReader<Snap>,
    resp: Query<&REntity, (With<SnapPoint>, Without<RPhantom>)>,
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
pub fn update_rphantom(
    pr: Res<PhantomSnaps>,
    preg: Res<PhantomREntityGeo>,
    mut ewp: Query<(&mut Transform, &mut REntity), (With<RPhantom>, Without<bevy_pancam::PanCam>)>,
    w: Query<&Window, With<PrimaryWindow>>,
    transform: Query<(&Camera, &GlobalTransform), With<bevy_pancam::PanCam>>,
) {
    let (ca, gt) = transform.single();
    for (tr, re) in ewp.iter_mut() {
        let tr = tr.into_inner();
        let re = re.into_inner();
        match re {
            REntity::Arc(sp) => update_rphantom_definition(&mut sp.definition, &preg),
            REntity::Circle(sp) => update_rphantom_definition(&mut sp.definition, &preg),
            REntity::Line(sp) => update_rphantom_definition(&mut sp.definition, &preg),
            REntity::Point(sp) => update_rphantom_point(&w, ca, gt, &pr, sp, tr),
            REntity::Text(sp) => update_rphantom_definition(&mut sp.definition, &preg),
        }
    }
}

pub fn update_rphantom_mesh(
    mut qmcre: Query<(&mut Mesh2dHandle, &REntity), With<RPhantom>>,
    mut me: ResMut<Assets<Mesh>>,
) {
    let lw = 0.3f32;

    for (m, re) in qmcre.iter_mut() {
        match re {
            REntity::Arc(_) => todo!(),
            REntity::Circle(_) => todo!(),
            REntity::Line(sp) => {
                let spec = sp.specifications();
                me.insert(&m.0, line_mesh(lw, spec.length, spec.h_angle));
            }
            REntity::Point(_) => (),
            REntity::Text(_) => todo!(),
        }
    }
}

fn update_rphantom_definition(sp: &mut [Point], preg: &Res<PhantomREntityGeo>) {
    let mut a = preg.definition.iter();
    for p in sp {
        if let Some(c) = a.next() {
            p.xy_mut(c.x, c.y);
        }
    }
}

fn update_rphantom_point(
    w: &Query<&Window, With<PrimaryWindow>>,
    ca: &Camera,
    gt: &GlobalTransform,
    pr: &Res<PhantomSnaps>,
    sp: &mut Point,
    tr: &mut Transform,
) {
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
// #[allow(clippy::type_complexity)]
// pub fn update_phantom_point(
//     pr: Res<PhantomSnaps>,
//     mut ewp: Query<(&mut Transform, &mut REntity), (With<RPhantom>, Without<bevy_pancam::PanCam>)>,
//     window: Query<&Window, With<PrimaryWindow>>,
//     transform: Query<(&Camera, &GlobalTransform), With<bevy_pancam::PanCam>>,
// ) {
//     let (ca, gt) = transform.single();
//     if let Ok((mut tr, re)) = ewp.get_single_mut() {
//         if let Some(cursor_world_pos) = window
//             .single()
//             .cursor_position()
//             .and_then(|cursor_pos| ca.viewport_to_world_2d(gt, cursor_pos))
//         {
//             let binding_xy = match pr.snap_to {
//                 Some(sv) => sv,
//                 None => cursor_world_pos,
//             };

//             re.into_inner()
//                 .unwrap_point_mut()
//                 .xy_mut(binding_xy.x, binding_xy.y);
//             tr.translation = Vec3::new(binding_xy.x, binding_xy.y, tr.translation.z);
//         }
//     }
// }
