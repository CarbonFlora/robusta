use bevy::prelude::*;
use robusta_gui::uistate::{show_ui_system, UiState};

use crate::test::*;

pub fn bootstrap() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin)
        .add_plugins(bevy_egui::EguiPlugin)
        .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        .add_plugins(bevy_pancam::PanCamPlugin::default())
        .insert_resource(UiState::new())
        .add_systems(Startup, pancam_setup)
        .add_systems(Update, draw_cursor)
        .add_systems(Update, draw_arc)
        .add_systems(PostUpdate, show_ui_system) // Currently broken, as I'm waiting for bevy_egui 0.25. Compiles, but sub-resource failure.
        // .add_systems(Update, keyboard_events)
        .run();
}

/// Currently only 1 viewport is supported.
fn pancam_setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(bevy_pancam::PanCam {
            zoom_to_cursor: false,
            ..default()
        });
}

fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    gizmos.circle_2d(point, 10., Color::WHITE);
}

// fn keyboard_events(
//     // mut key_event: EventReader<KeyboardInput>,
//     mut query: Query<&mut PanCam>,
//     keys: Res<Input<ScanCode>>
// ) {
//     if keys.just_pressed(ScanCode(36)) {
//         for mut pancam in &mut query {
//         }
//     }

//     // use bevy::input::ButtonState;

//     // for ev in key_event.read() {
//     //     match ev.state {
//     //         ButtonState::Pressed => {
//     //             println!("Key press: {:?} ({})", ev.key_code, ev.scan_code);

//     //         }
//     //         ButtonState::Released => {
//     //             println!("Key release: {:?} ({})", ev.key_code, ev.scan_code);
//     //         }
//     //     }
//     // }
// }
