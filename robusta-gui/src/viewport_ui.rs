use robusta_core::{line::Line, point::Point, RobustaEntity};

use super::*;

pub fn update_viewport_ui(
    act_write: EventWriter<Act>,
    mut ui_state: ResMut<UiState>,
    egui_context_primary: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut camera: Query<&mut bevy_pancam::PanCam>,
    window: Query<&Window, With<PrimaryWindow>>,
    transform: Query<(&Transform, &GlobalTransform), With<bevy_pancam::PanCam>>,
    secondary_window: Query<&mut Window, Without<PrimaryWindow>>,
) {
    match secondary_window.single().focused {
        true => {
            camera.single_mut().enabled = false;
        }
        false => {
            camera.single_mut().enabled = true;
        }
    }

    if ui_state.cad_state.cad_term.is_some() {
        update_terminal_egui(act_write, &mut ui_state, egui_context_primary);
    }

    if let Some((a, b)) = &mut ui_state.cad_state.construction {
        match b {
            RobustaEntity::Point(b) => update_construction_point(a, b, window, transform),
            RobustaEntity::Line(b) => place_line(b),
            _ => (),
        }
    }
}

fn update_construction_point(
    id: &Entity,
    target: &mut Point,
    window: Query<&Window, With<PrimaryWindow>>,
    transform: Query<(&Transform, &GlobalTransform), With<bevy_pancam::PanCam>>,
) {
    // from cursor global transform position, update the target and id entity x and y coordinates.
    // On click, remove
}

fn place_line(target: &mut Line) {}
