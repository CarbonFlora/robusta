use super::*;

#[allow(clippy::too_many_arguments)]
pub fn update_viewport_ui(
    aw: EventWriter<Act>,
    mut uis: ResMut<UiState>,
    ecp: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut bpc: Query<&mut bevy_pancam::PanCam>,
    sw: Query<&mut Window, Without<PrimaryWindow>>,
) {
    bpc.single_mut().enabled = !sw.single().focused;

    if uis.cad_state.cad_term.is_some() {
        update_terminal_egui(aw, &mut uis, ecp);
    }
}
