use bevy::{
    input::keyboard::KeyboardInput,
    prelude::{EventReader, KeyCode, ResMut},
};

use crate::uistate::UiState;

pub fn key_bindings(mut _ui_state: ResMut<UiState>, mut key_evr: EventReader<KeyboardInput>) {
    if let Some(a) = key_evr.read().next() {
        match a.key_code.unwrap_or_else(|| KeyCode::Numpad0) {
            KeyCode::ControlLeft | KeyCode::ControlRight => ctrl_functions(key_evr.read().next()),
            KeyCode::Colon => (),
            _ => (),
        }
    }
    // let keys = [None; 2];
    // if let Some(a) = key_evr.read().next() {
    //     if let Some(b) = a.key_code {
    //         keys[0] = Some(b);
    //     }
    // }
    // if let Some(a) = key_evr.read().next() {
    //     if let Some(b) = a.key_code {
    //         keys[1] = Some(b);
    //     }
    // }

    // if let Some(a) = keys[0] {

    //     match keys {
    //         KeyCode::ControlLeft | KeyCode::ControlRight => (),
    //         KeyCode::Colon => (),
    //         _ => (),
    //     }
    // }
}

fn ctrl_functions(key: Option<&KeyboardInput>) {}
