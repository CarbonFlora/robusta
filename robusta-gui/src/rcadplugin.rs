use self::{
    construction::ConstructionPlugin, diagnostic::DiagnosticPlugin, phantom::PhantomPlugin,
    rselection::RSelectionPlugin,
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
            .add_systems(Startup, camera_startup)
            .add_systems(Startup, spawn_window)
            .add_systems(PostStartup, draw_first)
            .add_systems(PreUpdate, capture_keystrokes)
            .add_systems(Update, update_viewport_ui)
            .add_systems(Update, update_dock)
            .add_systems(PostUpdate, update_act);
    }
}

/// Spawn a new window with reasonable defaults.
fn spawn_window(mut co: Commands) {
    co.spawn((
        window::Window {
            title: String::from("CADPanel"),
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
