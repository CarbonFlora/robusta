use crate::*;

use crate::test::draw_arc; //this is for testing only

pub fn app2d() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_plugins(bevy_egui::EguiPlugin)
        .add_plugins(bevy_pancam::PanCamPlugin::default())
        // .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin)
        // .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        .insert_resource(UiState::new())
        .add_systems(Startup, pancam_setup)
        .add_systems(First, draw_arc)
        .add_systems(Update, show_ui_system)
        .add_systems(Update, unfreeze_camera_viewport)
        .add_systems(PostUpdate, set_camera_viewport)
        // .add_systems(Update, keyboard_events)
        .run();
}

pub fn _ecs_schedule() -> crate::Schedule {
    let schedule = crate::Schedule::default();
    schedule
}
