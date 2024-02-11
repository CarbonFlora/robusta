use crate::leaves::term::run_cadt;

use super::*;

pub fn update_viewport_ui(
    mut ui_state: ResMut<UiState>,
    egui_context_primary: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut camera: Query<&mut bevy_pancam::PanCam>,
    secondary_window: Query<&mut Window, Without<PrimaryWindow>>,
    deselections: EventWriter<Pointer<Deselect>>,
) {
    match secondary_window.single().focused {
        true => {
            camera.single_mut().enabled = false;
        }
        false => {
            camera.single_mut().enabled = true;
        }
    }

    if ui_state.cad_state.cad_term.0 {
        open_term_egui(&mut ui_state, egui_context_primary);
    }

    match &ui_state.actions {
        Actions::OpenCADTerm => ui_state.cad_state.cad_term = (true, String::new()),
        Actions::TryOpen(input) => run_cadt(&ui_state, &input, deselections),
        Actions::Exit => ui_state.cad_state.close_all(),
        _ => (),
    }
}
