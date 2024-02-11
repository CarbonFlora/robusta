use egui::{Align2, TextEdit, Vec2};

use crate::*;

use self::{keystrokes::Actions, uistate::UiState};

pub fn open_term_egui(
    ui_state: &mut UiState,
    context: Query<&mut EguiContext, With<PrimaryWindow>>,
) {
    if let Ok(w) = context.get_single() {
        egui::Window::new("CADTerminal")
            .title_bar(false)
            .anchor(Align2::CENTER_BOTTOM, Vec2 { x: 0., y: 0. })
            .show(w.get(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("CADT: ");
                    let response = ui.add(
                        TextEdit::singleline(&mut ui_state.cad_state.cad_term.1)
                            .hint_text("Enter a command."),
                    );

                    if response.lost_focus() {
                        ui_state.cad_state.cad_term.0 = false;
                        ui_state.actions = Actions::TryOpen(ui_state.cad_state.cad_term.1.clone());
                        return;
                    }

                    response.request_focus();
                });
            });
    }
}

pub fn run_cadt(ui_state: &UiState, input: &String, deselections: EventWriter<Pointer<Deselect>>) {
    match input.as_str() {
        "dsa" => deselect_all(ui_state, deselections),
        _ => (),
    }
}

fn deselect_all(ui_state: &UiState, mut deselections: EventWriter<Pointer<Deselect>>) {
    for i in &ui_state.selected_entities {
        deselections.send(Pointer::new(i.1, i.2.clone(), i.0, Deselect))
    }
}
