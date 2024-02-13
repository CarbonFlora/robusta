use bevy::prelude::*;
use bevy_mod_picking::{prelude::Pointer, selection::Deselect};

use crate::{keystrokes::Act, uistate::UiState};

pub fn update_act(
    mut act_read: EventReader<Act>,
    mut ui_state: ResMut<UiState>,
    mut deselections: EventWriter<Pointer<Deselect>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    for act in act_read.read() {
        if let Act::TryAct(a) = act {
            run_act(
                &to_act(a),
                &mut ui_state,
                &mut deselections,
                &mut app_exit_events,
            );
        } else {
            run_act(act, &mut ui_state, &mut deselections, &mut app_exit_events)
        }
    }
}

fn run_act(
    act: &Act,
    ui_state: &mut ResMut<UiState>,
    deselections: &mut EventWriter<Pointer<Deselect>>,
    app_exit_events: &mut ResMut<Events<bevy::app::AppExit>>,
) {
    match act {
        // Act::NewPoint => new_point(
        //     &mut commands,
        //     &mut meshes,
        //     &mut materials,
        //     ui_state.cad_state.construction,
        // ),
        Act::DeselectAll => deselect_all(&ui_state, deselections),
        Act::OpenCADTerm => ui_state.cad_state.cad_term = Some(String::new()),
        Act::Exit => ui_state.cad_state.close_all(),
        Act::QuitWithoutSaving => app_exit_events.send(bevy::app::AppExit),
        _ => (),
    }
}

pub fn deselect_all(ui_state: &UiState, deselections: &mut EventWriter<Pointer<Deselect>>) {
    for i in &ui_state.selected_entities {
        deselections.send(Pointer::new(i.1, i.2.clone(), i.0, Deselect))
    }
}

fn to_act(input: &String) -> Act {
    return match input.as_str() {
        "deselect" | "dsa" => Act::DeselectAll,
        "point" | "p" => Act::NewPoint,
        "q!" => Act::QuitWithoutSaving,
        _ => Act::None,
    };
}
