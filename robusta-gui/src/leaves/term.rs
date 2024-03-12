use super::*;

pub fn update_terminal_egui(
    act_write: &mut EventWriter<Act>,
    ui_state: &mut UiState,
    context: &mut Query<&mut EguiContext, With<PrimaryWindow>>,
) {
    if let Ok(w) = context.get_single() {
        ui_state.cad_state.mode = Mode::Typing;
        egui::Window::new("CADTerminal")
            .title_bar(false)
            .anchor(Align2::CENTER_BOTTOM, Vec2 { x: 0., y: 0. })
            .show(w.get(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("CADT: ");
                    let response = ui.add(
                        TextEdit::singleline(ui_state.cad_state.cad_term.as_mut().unwrap())
                            .hint_text("Enter a command."),
                    );

                    if response.lost_focus() {
                        act_write.send(Act::TryAct(ui_state.cad_state.cad_term.clone().unwrap()));
                        ui_state.cad_state.mode = Mode::Normal;
                        ui_state.cad_state.cad_term = None;
                        return;
                    }

                    response.request_focus();
                });
            });
    }
}
