use crate::*;

// use crate::test::draw_arc; //this is for testing only

pub fn app2d(path: Option<String>) {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_plugins(bevy_egui::EguiPlugin)
        .add_plugins(bevy_pancam::PanCamPlugin::default())
        // .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        // .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin)
        .insert_resource(UiState::new())
        .insert_resource(ViewportState::new(path))
        .add_systems(Startup, pancam_setup)
        // .add_systems(First, draw_arc)
        .add_systems(First, draw_dxf)
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

fn pancam_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert((
        bevy_pancam::PanCam {
            zoom_to_cursor: false,
            ..default()
        },
        // RaycastPickCamera::default(), This is set automatically.
    ));
}
