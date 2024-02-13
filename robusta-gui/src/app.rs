use super::*;

pub fn app2d(path: Option<String>) {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_plugins(bevy_egui::EguiPlugin)
        .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        .add_plugins(bevy_pancam::PanCamPlugin::default())
        .insert_resource(UiState::new(&path))
        .insert_resource(EntityMapping::new())
        .add_event::<SelectionInstance>()
        .add_event::<Act>()
        .add_systems(Startup, camera_startup)
        .add_systems(Startup, spawn_window)
        .add_systems(PostStartup, draw_first)
        .add_systems(PreUpdate, capture_keystrokes)
        .add_systems(Update, update_viewport_ui)
        .add_systems(Update, update_dock)
        .add_systems(PostUpdate, update_act)
        .run();
}

/// Spawn a new window with reasonable defaults.
pub fn spawn_window(mut commands: Commands) {
    commands.spawn((window::Window::default(), CADPanel::default()));
}

// Spawn a camera & configures bevy_mod_picking.
pub fn camera_startup(
    mut commands: Commands,
    mut selection_settings: ResMut<selection::SelectionSettings>,
) {
    selection_settings.click_nothing_deselect_all = false;
    commands
        .spawn(Camera2dBundle::default())
        .insert((bevy_pancam::PanCam {
            grab_buttons: vec![MouseButton::Middle, MouseButton::Right],
            zoom_to_cursor: false,
            ..default()
        },));
}
