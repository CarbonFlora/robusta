use egui::{Button, Ui};

use super::*;

pub fn update_insert_egui(
    ewa: &mut EventWriter<Act>,
    context: &mut Query<&mut EguiContext, With<PrimaryWindow>>,
) {
    let w = match context.get_single() {
        Ok(w) => w,
        Err(_) => return,
    };

    egui::Window::new("Insert")
        .title_bar(false)
        .anchor(Align2::CENTER_BOTTOM, Vec2 { x: 0., y: 0. })
        .show(w.get(), |ui| {
            ui.label("Insert");
            ui.horizontal(|ui| {
                construct_type_button(ui, ewa, ConstructType::Point);
                construct_type_button(ui, ewa, ConstructType::Line);
                construct_type_button(ui, ewa, ConstructType::Arc);
                construct_type_button(ui, ewa, ConstructType::Circle);
                construct_type_button(ui, ewa, ConstructType::Text);

                // let response = ui.add(
                //     TextEdit::singleline(ui_state.cad_state.cad_term.as_mut().unwrap())
                //         .hint_text("Enter a command."),
                // );

                // if response.lost_focus() {
                //     act_write.send(Act::TryAct(ui_state.cad_state.cad_term.clone().unwrap()));
                //     ui_state.cad_state.mode = Mode::Normal;
                //     ui_state.cad_state.cad_term = None;
                //     return;
                // }

                // response.request_focus();
            });
        });
}

fn construct_type_button(ui: &mut Ui, ewa: &mut EventWriter<Act>, ct: ConstructType) {
    if ui.add(Button::new(format!("{ct}"))).clicked() {
        ewa.send(Act::Insert(Some(ct)));
    }
}
