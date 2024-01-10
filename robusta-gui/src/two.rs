use bevy::prelude::*;
// use bevy::input::keyboard::KeyboardInput;
use bevy_pancam::{PanCamPlugin, PanCam};


use crate::test::*;

pub fn startup_bevy_2d() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(PanCamPlugin::default())
    .add_systems(Startup, pancam_setup)
    .add_systems(Update, draw_cursor)
    .add_systems(Update, draw_arc)
    // .add_systems(Update, keyboard_events)
    .run();
}

fn pancam_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default())
    .insert(PanCam {zoom_to_cursor: false, ..default()});
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