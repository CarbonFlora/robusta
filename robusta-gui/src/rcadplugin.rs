use super::*;

pub struct RCADPlugins;

impl bevy::app::PluginGroup for RCADPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut builder = bevy::app::PluginGroupBuilder::start::<Self>();

        builder = builder.add(RCADCorePlugin);

        builder
    }
}

pub struct RCADCorePlugin;
impl bevy::app::Plugin for RCADCorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(EntityMapping::new())
            .add_event::<SelectionInstance>()
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
fn spawn_window(mut commands: Commands) {
    commands.spawn((window::Window::default(), CADPanel::default()));
}

fn camera_startup(
    mut commands: Commands,
    mut selection_settings: ResMut<selection::SelectionSettings>,
    debug_picking: ResMut<State<bevy_mod_picking::debug::DebugPickingMode>>,
) {
    selection_settings.click_nothing_deselect_all = false;
    *debug_picking.into_inner() = State::new(bevy_mod_picking::debug::DebugPickingMode::Disabled);

    commands
        .spawn(Camera2dBundle::default())
        .insert((bevy_pancam::PanCam {
            grab_buttons: vec![MouseButton::Middle, MouseButton::Right],
            zoom_to_cursor: false,
            ..default()
        },));
}
