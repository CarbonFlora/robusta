use bevy::{
    input::keyboard::KeyboardInput,
    prelude::{EventReader, KeyCode, Query, ResMut, With, World},
};
use bevy_egui::EguiContext;
use bevy_window::PrimaryWindow;

use crate::{
    leaves::term::{open_term, open_term_egui},
    uistate::UiState,
};

pub fn pressed_keys(
    context: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut ui_state: ResMut<UiState>,
    mut key_evr: EventReader<KeyboardInput>,
) {
    // let mut buffer = [None; 2];
    let mut buffer = ui_state.pressed_keys;
    for (i, key_input) in key_evr.read().take(2).enumerate() {
        buffer[i] = key_input.key_code;
    }

    ui_state.pressed_keys = buffer;
}

fn ctrl_functions(key: Option<&KeyboardInput>) {
    if let Some(w) = key {
        match w.key_code.unwrap_or_else(|| KeyCode::Numpad0) {
            KeyCode::Colon => (),
            _ => (),
        }
    }
}
