use self::{
    phantom::RPhantomPointer, point::Point, selection::PickableSelectionBundle, snap::SnapBundle,
};

use super::*;

pub struct UnsortedPlugin;
impl bevy::app::Plugin for UnsortedPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(TopZLayer::new())
            .add_event::<Act>()
            .add_event::<REntity>()
            .add_systems(Startup, camera_startup)
            .add_systems(Startup, spawn_window)
            .add_systems(PostStartup, draw_first)
            .add_systems(PreUpdate, capture_keystrokes)
            .add_systems(Update, update_viewport_ui)
            .add_systems(Update, update_dock)
            .add_systems(Update, update_spawn_rentities)
            .add_systems(PreUpdate, update_act);
    }
}

/// Spawn a new window with reasonable defaults.
fn spawn_window(mut co: Commands) {
    co.spawn((
        window::Window {
            title: String::from("CADPanel"),
            // present_mode: bevy_window::PresentMode::AutoNoVsync,
            focused: false,
            ..Default::default()
        },
        CADPanel::default(),
    ));
}

fn camera_startup(mut co: Commands, dp: ResMut<bevy_mod_picking::debug::DebugPickingMode>) {
    *dp.into_inner() = bevy_mod_picking::debug::DebugPickingMode::Disabled;

    co.spawn(Camera2dBundle::default())
        .insert((bevy_pancam::PanCam {
            grab_buttons: vec![MouseButton::Middle, MouseButton::Right],
            // zoom_to_cursor: false,
            ..default()
        },));
}

pub fn update_spawn_rentities(
    //Input
    mut erre: EventReader<REntity>,
    //Util
    mut me: ResMut<Assets<Mesh>>,
    mut ma: ResMut<Assets<ColorMaterial>>,
    mut tz: ResMut<TopZLayer>,
    //Output
    mut co: Commands,
) {
    for re in erre.read() {
        match re {
            REntity::Arc(sp) => co.spawn((
                REntity::Arc(sp.clone()),
                sp.mesh(&mut me, &mut ma, &mut tz),
                PickableSelectionBundle::default(),
            )),
            REntity::Circle(sp) => co.spawn((
                REntity::Circle(sp.clone()),
                sp.mesh(&mut me, &mut ma, &mut tz),
                PickableSelectionBundle::default(),
            )),
            REntity::Line(sp) => co.spawn((
                REntity::Line(sp.clone()),
                sp.mesh(&mut me, &mut ma, &mut tz),
                PickableSelectionBundle::default(),
            )),
            REntity::Point(sp) => co.spawn((
                REntity::Point(sp.clone()),
                sp.mesh(&mut me, &mut ma, &mut tz),
                PickableSelectionBundle::default(),
            )),
            REntity::Text(sp) => co.spawn((
                REntity::Text(sp.clone()),
                // sp.mesh(&mut me, &mut ma, &mut tz),
                sp.text_mesh(&mut tz),
                PickableSelectionBundle::default(),
            )),
            REntity::PhantomPoint => co.spawn((
                REntity::Point(Point::new(0., 0., 0.)),
                Point::new(0., 0., 0.).mesh(&mut me, &mut ma, &mut tz),
                RPhantomPointer,
            )),
            REntity::SnapPoint(sp) => co.spawn((
                REntity::Point(sp.clone()),
                sp.mesh(&mut me, &mut ma, &mut tz),
                SnapBundle::default(),
            )),
        };
    }
}

// fn spawn_snap_bundle(
//     vp: point::Point,
//     co: &mut Commands,
//     me: &mut ResMut<Assets<Mesh>>,
//     ma: &mut ResMut<Assets<ColorMaterial>>,
//     xi: &mut ResMut<TopZLayer>,
// ) -> Entity {
//     co.spawn((
//         MaterialMesh2dBundle {
//             mesh: me.add(bevy::math::primitives::Circle::new(0.2)).into(),
//             material: ma.add(ColorMaterial::from(Color::ORANGE)),
//             transform: Transform::from_translation(Vec3::new(
//                 vp.coordinates.x,
//                 vp.coordinates.y,
//                 xi.top() as f32,
//             )),
//             ..default()
//         },
//         SnapPoint,
//         REntity::Point(vp),
//         On::<Pointer<Over>>::send_event::<Snap>(),
//         On::<Pointer<Out>>::send_event::<Snap>(),
//     ))
//     .id()
// }

// fn spawn_phantom_bundle(
//     co: &mut Commands,
//     me: &mut ResMut<Assets<Mesh>>,
//     ma: &mut ResMut<Assets<ColorMaterial>>,
//     xi: &mut ResMut<TopZLayer>,
// ) -> Entity {
//     co.spawn((
//         MaterialMesh2dBundle {
//             mesh: me.add(bevy::math::primitives::Circle::new(0.5)).into(),
//             material: ma.add(ColorMaterial::from(Color::CYAN)),
//             transform: Transform::from_translation(Vec3::new(0., 0., xi.top() as f32)),
//             ..default()
//         },
//         REntity::Point(point::Point::new(0., 0., 0.)),
//         RPhantomPointer,
//     ))
//     .id()
// }
