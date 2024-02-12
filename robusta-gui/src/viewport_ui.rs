use super::*;

pub fn update_viewport_ui(
    act_write: EventWriter<Act>,
    mut ui_state: ResMut<UiState>,
    egui_context_primary: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut camera: Query<&mut bevy_pancam::PanCam>,
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
        open_term_egui(act_write, &mut ui_state, egui_context_primary);
    }
}
