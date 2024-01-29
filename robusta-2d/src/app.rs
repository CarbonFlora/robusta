use robusta_exp::camera_startup;
use robusta_exp::egui_window;

use crate::*;

pub fn app2d(path: Option<String>) {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_plugins(bevy_egui::EguiPlugin)
        .add_plugins(bevy_pancam::PanCamPlugin::default())
        .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin)
        .insert_resource(UiState::new(&path))
        .add_systems(Startup, camera_startup)
        .add_systems(Startup, egui_window)
        .add_systems(First, draw_first)
        // .add_systems(Update, unfreeze_camera_viewport)
        // .add_systems(PostUpdate, update_camera_viewport)
        // .add_systems(Update, keyboard_events)
        .run();
}
