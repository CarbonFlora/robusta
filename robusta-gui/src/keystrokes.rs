use bevy::prelude::{Input, KeyCode, Res, ResMut};

use crate::uistate::UiState;

pub fn capture_keystrokes(
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
            Actions::OpenCADTerm
        }
        _ => Actions::None,
    }
}

#[derive(Debug, PartialEq)]
pub enum Actions {
    None,
    Exit,
    OpenCADTerm,
    TryOpen(String),
}
