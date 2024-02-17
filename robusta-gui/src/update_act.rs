use bevy::prelude::*;
use bevy_mod_picking::{prelude::Pointer, selection::Deselect};

use crate::{keystrokes::Act, uistate::UiState, EntityMapping};

pub fn update_act(
    mut act_read: EventReader<Act>,
    mut ui_state: ResMut<UiState>,
    entity_mapping: Res<EntityMapping>,
    mut deselections: EventWriter<Pointer<Deselect>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut camera: Query<(&mut Transform, &GlobalTransform), With<bevy_pancam::PanCam>>,
) {
    for act in act_read.read() {
        if let Act::TryAct(a) = act {
            run_act(
                &to_act(a),
                &mut ui_state,
                &entity_mapping,
                &mut deselections,
                &mut app_exit_events,
                &mut camera,
            );
        } else {
            run_act(
                act,
                &mut ui_state,
                &entity_mapping,
                &mut deselections,
                &mut app_exit_events,
                &mut camera,
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
    camera: &mut Query<(&mut Transform, &GlobalTransform), With<bevy_pancam::PanCam>>,
) {
    match act {
        Act::PullCameraFocus(rect) => camera_movement(rect, camera),
        Act::FitView => camera_movement(&ui_state.all_rect(), camera),
        Act::Inspect => ui_state.inspect(),
        Act::DeselectAll => ui_state.deselect_all(deselections),
        Act::OpenCADTerm => ui_state.cad_state.cad_term = Some(String::new()),
        Act::DebugReMapSelection(entity) => ui_state.remap_selection(entity, entity_mapping),
        Act::Exit => ui_state.cad_state.close_all(),
        Act::QuitWithoutSaving => app_exit_events.send(bevy::app::AppExit),
        _ => (),
    }
}

fn to_act(input: &str) -> Act {
    match input {
        "deselect" | "dsa" => Act::DeselectAll,
        "inspect" | "i" => Act::Inspect,
        "fitview" | "fv" => Act::FitView,
        "point" | "p" => Act::NewPoint,
        "q!" => Act::QuitWithoutSaving,
        _ => Act::None,
    }
}

fn camera_movement(
    entity_position: &Rect,
    camera: &mut Query<(&mut Transform, &GlobalTransform), With<bevy_pancam::PanCam>>,
) {
    let mut camera = camera.get_single_mut().unwrap();
    let current_3d_pos = camera.1.to_scale_rotation_translation().2;
    let current_2d_pos = Vec2::new(current_3d_pos.x, current_3d_pos.y);
    let entity_position = (entity_position.max - entity_position.min) / 2. + entity_position.min;
    let delta = current_2d_pos - entity_position;
    let proposed_cam_transform = camera.0.translation - delta.extend(0.);

    camera.0.translation = proposed_cam_transform;
}
