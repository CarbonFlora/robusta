use super::*;

pub fn app2d(path: Option<String>) {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Viewport".into(),
                // present_mode: bevy_window::PresentMode::Immediate,
                present_mode: bevy_window::PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..default()
        }))
        // .add_plugins(bevy_framepace::FramepacePlugin)
        .add_plugins(bevy_egui::EguiPlugin)
        .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        .add_plugins(bevy_pancam::PanCamPlugin)
        .insert_resource(UiState::new(&path))
        .add_plugins(RCADPlugins)
        .run();
}
