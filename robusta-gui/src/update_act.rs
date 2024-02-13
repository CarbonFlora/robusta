use bevy::prelude::*;
use bevy_mod_picking::{prelude::Pointer, selection::Deselect};

use crate::{keystrokes::Act, uistate::UiState, EntityMapping};

pub fn update_act(
    mut act_read: EventReader<Act>,
    mut ui_state: ResMut<UiState>,
    entity_mapping: Res<EntityMapping>,
    mut deselections: EventWriter<Pointer<Deselect>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    for act in act_read.read() {
        if let Act::TryAct(a) = act {
            run_act(
                &to_act(a),
                &mut ui_state,
                &entity_mapping,
                &mut deselections,
                &mut app_exit_events,
            );
        } else {
            run_act(
                act,
                &mut ui_state,
                &entity_mapping,
                &mut deselections,
                &mut app_exit_events,
            )
        }
    }
}

fn run_act(
    act: &Act,
    ui_state: &mut ResMut<UiState>,
    entity_mapping: &Res<EntityMapping>,
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
        // Act::DeselectAll => deselect_all(&ui_state, deselections),
        Act::DeselectAll => ui_state.deselect_all(deselections),
        Act::OpenCADTerm => ui_state.cad_state.cad_term = Some(String::new()),
        Act::DebugReMapSelection(entity) => ui_state.remap_selection(entity, entity_mapping),
        Act::Exit => ui_state.cad_state.close_all(),
        Act::QuitWithoutSaving => app_exit_events.send(bevy::app::AppExit),
        _ => (),
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
