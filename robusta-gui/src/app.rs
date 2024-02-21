use super::*;

pub fn app2d(path: Option<String>) {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_plugins(bevy_egui::EguiPlugin)
        .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        .add_plugins(bevy_pancam::PanCamPlugin)
        .insert_resource(UiState::new(&path))
        .add_plugins(self::rcadplugin::RCADPlugins)
        .run();
}
