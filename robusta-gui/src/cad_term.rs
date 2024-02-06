use bevy::prelude::{Input, KeyCode, Res, ResMut};

use crate::uistate::UiState;

// pub fn pressed_keys(
//     context: Query<&mut EguiContext, With<PrimaryWindow>>,
//     mut ui_state: ResMut<UiState>,
//     mut key_evr: EventReader<KeyboardInput>,
// ) {
//     // let mut buffer = [None; 2];
//     let mut buffer = ui_state.pressed_keys;
//     for (i, key_input) in key_evr.read().take(2).enumerate() {
//         buffer[i] = key_input.key_code;
//     }

//     ui_state.pressed_keys = buffer;
// }

// fn ctrl_functions(key: Option<&KeyboardInput>) {
//     if let Some(w) = key {
//         match w.key_code.unwrap_or_else(|| KeyCode::Numpad0) {
//             KeyCode::Colon => (),
//             _ => (),
//         }
//     }
// }

pub fn pressed_keys(
    // context: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut ui_state: ResMut<UiState>,
    keys: Res<Input<KeyCode>>,
) {
    let mut buffer = [None; 2];

    for keycode in keys.get_pressed() {
        match keycode {
            KeyCode::ControlLeft | KeyCode::ControlRight => buffer[0] = Some(KeyCode::ControlLeft),
            KeyCode::ShiftLeft | KeyCode::ShiftRight => buffer[0] = Some(KeyCode::ShiftLeft),
            KeyCode::AltLeft | KeyCode::AltRight => buffer[0] = Some(KeyCode::AltLeft),
            _ => buffer[1] = Some(keycode.clone()),
        };
    }

    ui_state.pressed_keys = buffer;

    ui_state.actions = match buffer {
        [None, Some(KeyCode::Escape)] => Actions::Exit,
        [None, Some(KeyCode::Semicolon)] | [Some(KeyCode::ShiftLeft), Some(KeyCode::Semicolon)] => {
            Actions::ToggleCADTerm
        }
        _ => Actions::None,
    }
}

#[derive(Debug)]
pub enum Actions {
    None,
    Exit,
    ToggleCADTerm,
}
