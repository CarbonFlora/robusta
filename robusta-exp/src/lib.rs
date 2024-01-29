use bevy::window;

use bevy::prelude::*;
// use bevy_mod_picking::backends::egui::EguiPointer;
// use bevy_mod_picking::prelude::*;

/// Spawn a new window with reasonable defaults.
pub fn egui_window(mut commands: Commands) {
    commands.spawn(window::Window::default());
}

// Spawn a camera. Two cameras should not be assigned to the same viewport.
pub fn camera_startup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert((bevy_pancam::PanCam {
            zoom_to_cursor: false,
            ..default()
        },));
}
