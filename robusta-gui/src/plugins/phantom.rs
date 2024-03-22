use crate::REntity;

use self::{
    construction::ConstructionInput,
    point::Point,
    snap::{SnapPoint, SnapTo},
};

use super::*;

pub struct PhantomPlugin;
impl bevy::app::Plugin for PhantomPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(PhantomSnaps::new())
            .add_event::<PhantomAct>()
            .add_systems(PreUpdate, update_phantom_snap)
            .add_systems(Update, update_rphantom_pointer_location)
            .add_systems(Update, update_phantoms);
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

#[derive(Debug, Event)]
pub enum PhantomAct {
    DespawnAll,
}

/// This is a marker component to delineate a point entity in the process of being placed.
#[derive(Debug, Component)]
pub struct RPhantomPointer;

/// This is an additional marker component to delineate a static phantom point.
#[derive(Debug, Component)]
pub struct RPhantomStatic;

pub fn update_phantoms(
    //Input
    mut erpa: EventReader<PhantomAct>,
    //Output
    mut co: Commands,
    qewrpp: Query<Entity, With<RPhantomPointer>>,
    qewrps: Query<Entity, With<RPhantomStatic>>,
    mut rmps: ResMut<PhantomSnaps>,
) {
    for pa in erpa.read() {
        match pa {
            PhantomAct::DespawnAll => {
                for e in qewrpp.iter() {
                    co.entity(e).remove::<RPhantomPointer>();
                    co.entity(e).despawn();
                }
                for e in qewrps.iter() {
                    co.entity(e).remove::<RPhantomStatic>();
                    co.entity(e).despawn();
                }
                rmps.snap_to = None;
            }
        }
    }
}

pub fn update_phantom_snap(
    //Input
    mut ers: EventReader<SnapTo>,
    //Output
    mut pr: ResMut<PhantomSnaps>,
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
pub fn update_rphantom_pointer_location(
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
    ewre: &mut EventWriter<REntity>,
) {
    let re = match qre.get_single() {
        Ok(w) => w,
        Err(_) => return,
    };
    let xyz = re.unwrap_point().coordinates;
    let coords = Vec3::new(xyz.x, xyz.y, xyz.z);
    let mut point = Point::new(coords.x, coords.y, coords.z);
    point.set_appearance(Color::FUCHSIA, 0.4);
    ewre.send(REntity::PhantomStatic(point));
    ewci.send(ConstructionInput { coords });
}
