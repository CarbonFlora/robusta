use bevy::{
    app::{PreUpdate, Update},
    ecs::{
        component::Component,
        entity::Entity,
        event::{EventReader, EventWriter},
        query::{With, Without},
        system::{Commands, Query, Res, ResMut, Resource},
    },
    math::{Vec2, Vec3},
    prelude::default,
    render::{
        camera::Camera,
        color::Color,
        mesh::{shape, Mesh},
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    transform::components::{GlobalTransform, Transform},
};
use bevy_asset::Assets;
use bevy_mod_picking::{
    events::Pointer,
    prelude::On,
    selection::{Deselect, Select},
    PickableBundle,
};
use bevy_window::{PrimaryWindow, Window};
use robusta_core::point;

use crate::{
    rselection::Selection,
    snap::{Snap, SnapPoint, UpdateSnapPoints},
    REntity, TopZLayer,
};

pub struct PhantomPlugin;
impl bevy::app::Plugin for PhantomPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(PhantomResources::new())
            .add_systems(PreUpdate, update_phantom_snap)
            .add_systems(Update, update_phantom_point);
    }
}

#[derive(Debug, Resource, Default)]
pub struct PhantomResources {
    snap_to: Option<Vec2>,
}

impl PhantomResources {
    pub fn new() -> Self {
        PhantomResources::default()
    }
}

/// This is a marker component to delineate a point entity in the process of being placed.
#[derive(Debug, Component)]
pub struct PhantomPoint;

pub fn despawn_all_phantoms(c: &mut Commands, ewp: &Query<Entity, With<PhantomPoint>>) {
    for e in ewp.iter() {
        c.entity(e).despawn();
    }
}

pub fn canonize(
    c: &mut Commands,
    ewp: &Query<Entity, With<PhantomPoint>>,
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
    c.entity(e).remove::<PhantomPoint>();
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
            mesh: me.add(shape::Circle::new(0.5).into()).into(),
            material: ma.add(ColorMaterial::from(Color::CYAN)),
            transform: Transform::from_translation(Vec3::new(0., 0., tzi.top() as f32)),
            ..default()
        },
        REntity::Point(point::Point::new(0., 0., 0.)),
        PhantomPoint,
    ));
}

pub fn update_phantom_snap(
    mut pr: ResMut<PhantomResources>,
    mut ers: EventReader<Snap>,
    resp: Query<&REntity, (With<SnapPoint>, Without<PhantomPoint>)>,
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
pub fn update_phantom_point(
    pr: Res<PhantomResources>,
    mut ewp: Query<
        (&mut Transform, &mut REntity),
        (With<PhantomPoint>, Without<bevy_pancam::PanCam>),
    >,
    window: Query<&Window, With<PrimaryWindow>>,
    transform: Query<(&Camera, &GlobalTransform), With<bevy_pancam::PanCam>>,
) {
    let (ca, gt) = transform.single();
    if let Ok((mut tr, re)) = ewp.get_single_mut() {
        if let Some(cursor_world_pos) = window
            .single()
            .cursor_position()
            .and_then(|cursor_pos| ca.viewport_to_world_2d(gt, cursor_pos))
        {
            let binding_xy = match pr.snap_to {
                Some(sv) => sv,
                None => cursor_world_pos,
            };

            re.into_inner()
                .unwrap_point_mut()
                .xy_mut(binding_xy.x, binding_xy.y);
            tr.translation = Vec3::new(binding_xy.x, binding_xy.y, tr.translation.z);
        }
    }
}
