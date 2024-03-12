use self::leaves::insert::update_insert_egui;

use super::*;

#[allow(clippy::too_many_arguments)]
pub fn update_viewport_ui(
    mut aw: EventWriter<Act>,
    mut uis: ResMut<UiState>,
    mut ecp: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut bpc: Query<&mut bevy_pancam::PanCam>,
    sw: Query<&mut Window, Without<PrimaryWindow>>,
) {
    bpc.single_mut().enabled = !sw.single().focused;

    if uis.cad_state.cad_term.is_some() {
        update_terminal_egui(&mut aw, &mut uis, &mut ecp);
    }
    if uis.cad_state.insert_menu.is_some() {
        update_insert_egui(&mut aw, &mut uis, &mut ecp);
    }
}
