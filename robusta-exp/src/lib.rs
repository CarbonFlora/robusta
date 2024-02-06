// use bevy::window;
// use bevy_mod_picking::prelude::*;

// use bevy::prelude::*;
// use robusta_gui::uistate::CADPanel;
// // use bevy_mod_picking::backends::egui::EguiPointer;
// // use bevy_mod_picking::prelude::*;

// /// Spawn a new window with reasonable defaults.
// pub fn spawn_window(mut commands: Commands) {
//     commands.spawn((window::Window::default(), CADPanel::default()));
// }

// // Spawn a camera. Two cameras should not be assigned to the same viewport.
// pub fn camera_startup(mut commands: Commands) {
//     commands.spawn(Camera2dBundle::default()).insert((
//         bevy_pancam::PanCam {
//             grab_buttons: vec![MouseButton::Middle],
//             zoom_to_cursor: false,
//             ..default()
//         },
//         On::<Pointer<Down>>::send_event::<DoSomethingComplex>(),
//     ));
// }
