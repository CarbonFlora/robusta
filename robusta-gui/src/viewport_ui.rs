use self::leaves::{insert::update_insert_egui, snap::update_snap_egui};

use super::*;

#[allow(clippy::too_many_arguments)]
pub fn update_viewport_ui(
    mut aw: EventWriter<Act>,
    mut uis: ResMut<UiState>,
    mut ecp: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut ss: ResMut<SnapSettings>,
    mut db: ResMut<DockBuffer>,
) {
    if uis.cad_state.cad_term.is_some() {
        update_terminal_egui(&mut aw, &mut uis, &mut ecp);
    }
    if uis.cad_state.insert_menu.is_some() {
        update_insert_egui(&mut aw, &mut ecp);
    }
    if uis.cad_state.snap_menu.is_some() {
        update_snap_egui(&mut aw, &mut ecp, &mut ss, &mut db);
    }
}
