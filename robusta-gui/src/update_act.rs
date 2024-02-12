use bevy::prelude::*;
use bevy_mod_picking::{prelude::Pointer, selection::Deselect};

use crate::{keystrokes::Act, uistate::UiState};

pub fn update_act(
    mut act_read: EventReader<Act>,
    mut ui_state: ResMut<UiState>,
    mut deselections: EventWriter<Pointer<Deselect>>,
) {
    for act in act_read.read() {
        match act {
            Act::DeselectAll => deselect_all(&ui_state, &mut deselections),
            Act::OpenCADTerm => ui_state.cad_state.cad_term = (true, String::new()),
            Act::TryAct(input) => try_act(&ui_state, &input, &mut deselections),
            Act::Exit => ui_state.cad_state.close_all(),
            _ => (),
        }
    }
}

pub fn deselect_all(ui_state: &UiState, deselections: &mut EventWriter<Pointer<Deselect>>) {
    for i in &ui_state.selected_entities {
        deselections.send(Pointer::new(i.1, i.2.clone(), i.0, Deselect))
    }
}

fn try_act(ui_state: &UiState, input: &String, deselections: &mut EventWriter<Pointer<Deselect>>) {
    match input.as_str() {
        "deselect" | "dsa" => deselect_all(ui_state, deselections),
        _ => (),
    }
}
