use bevy::window;
use robusta_gui::cad_term::pressed_keys;
use robusta_gui::uistate::{cad_panel, CADPanel, DoSomethingComplex};

use crate::*;

pub fn app2d(path: Option<String>) {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_plugins(bevy_egui::EguiPlugin)
        .add_plugins(bevy_text_popup::TextPopupPlugin)
        .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        // .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin)
        .add_plugins(bevy_pancam::PanCamPlugin::default())
        .insert_resource(UiState::new(&path))
        .add_event::<DoSomethingComplex>()
        .add_systems(Startup, camera_startup)
        .add_systems(Startup, spawn_window)
        .add_systems(PostStartup, draw_first)
        .add_systems(PreUpdate, pressed_keys)
        .add_systems(Update, cad_panel)
        // .add_systems(Update, unfreeze_camera_viewport)
        // .add_systems(PostUpdate, update_camera_viewport)
        // .add_systems(Update, keyboard_events)
        .run();
}

/// Spawn a new window with reasonable defaults.
pub fn spawn_window(mut commands: Commands) {
    commands.spawn((window::Window::default(), CADPanel::default()));
}

// Spawn a camera. Two cameras should not be assigned to the same viewport.
pub fn camera_startup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert((bevy_pancam::PanCam {
            grab_buttons: vec![MouseButton::Middle],
            zoom_to_cursor: false,
            ..default()
        },));
}
