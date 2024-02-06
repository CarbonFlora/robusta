use super::*;

pub fn update_viewport_ui(
    mut ui_state: ResMut<UiState>,
    egui_context_primary: Query<&mut EguiContext, With<PrimaryWindow>>,
) {
    match ui_state.actions {
        Actions::OpenCADTerm => ui_state.cad_state.cad_term = (true, String::new()),
        Actions::Exit => ui_state.cad_state.close_all(),
        _ => (),
    }

    if ui_state.cad_state.cad_term.0 {
        open_term_egui(&mut ui_state, egui_context_primary);
    }
}
