use robusta_core::point::Point;

use self::{
    construction::ConstructionPlugin,
    diagnostic::DiagnosticPlugin,
    parse::dxf::line::spawn_line_mesh,
    phantom::PhantomPlugin,
    rselection::RSelectionPlugin,
    snap::{Snap, SnapPoint},
};

use super::*;

pub struct RCADPlugins;

impl bevy::app::PluginGroup for RCADPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut builder = bevy::app::PluginGroupBuilder::start::<Self>();

        builder = builder.add(RCADCorePlugin);
        builder = builder.add(RSelectionPlugin);
        builder = builder.add(SnapPlugin);
        builder = builder.add(PhantomPlugin);
        builder = builder.add(ConstructionPlugin);
        builder = builder.add(DiagnosticPlugin);

        builder
    }
}

pub struct RCADCorePlugin;
impl bevy::app::Plugin for RCADCorePlugin {
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
            // .add_systems(PostUpdate, update_act);
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
            REntity::Arc(_) => todo!(),
            REntity::Circle(_) => todo!(),
            REntity::Line(sp) => spawn_line_mesh(sp.clone(), &mut co, &mut me, &mut ma, &mut tz),
            REntity::Point(_) => todo!(),
            REntity::Text(_) => todo!(),
            REntity::SnapPoint(sp) => {
                spawn_snap_bundle(sp.clone(), &mut co, &mut me, &mut ma, &mut tz)
            }
        };
    }
}

fn spawn_snap_bundle(
    vp: Point,
    co: &mut Commands,
    me: &mut ResMut<Assets<Mesh>>,
    ma: &mut ResMut<Assets<ColorMaterial>>,
    xi: &mut ResMut<TopZLayer>,
) -> Entity {
    co.spawn((
        MaterialMesh2dBundle {
            mesh: me.add(bevy::math::primitives::Circle::new(0.2)).into(),
            material: ma.add(ColorMaterial::from(Color::ORANGE)),
            transform: Transform::from_translation(Vec3::new(
                vp.coordinates.x,
                vp.coordinates.y,
                xi.top() as f32,
            )),
            ..default()
        },
        SnapPoint,
        REntity::Point(vp),
        On::<Pointer<Over>>::send_event::<Snap>(),
        On::<Pointer<Out>>::send_event::<Snap>(),
    ))
    .id()
}
