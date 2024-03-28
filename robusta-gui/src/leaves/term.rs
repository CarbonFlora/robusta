use super::*;

pub fn update_terminal_egui(
    aw: &mut EventWriter<Act>,
    buffer: &mut String,
    context: &mut Query<&mut EguiContext, With<PrimaryWindow>>,
) {
    if let Ok(w) = context.get_single() {
        egui::Window::new("CADTerminal")
            .title_bar(false)
            .anchor(Align2::CENTER_BOTTOM, Vec2 { x: 0., y: 0. })
            .show(w.get(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("CADT: ");
                    let response =
                        ui.add(TextEdit::singleline(buffer).hint_text("Enter a command."));

                    if response.lost_focus() {
                        aw.send(Act::TryAct(buffer.clone()));
                        aw.send(Act::CameraUIMenu(CameraUiMenu::NoMenu));
                        return;
                    }

                    response.request_focus();
                });
            });
    }
}
