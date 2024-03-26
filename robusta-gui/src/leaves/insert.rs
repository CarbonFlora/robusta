use egui::{Button, Ui};

use super::*;

pub fn update_insert_egui(
    aw: &mut EventWriter<Act>,
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
                construct_type_button(ui, aw, ConstructType::PointBy1Click);
                construct_type_button(ui, aw, ConstructType::LineBy2Click);
                construct_type_button(ui, aw, ConstructType::ArcByEndEndMid);
                construct_type_button(ui, aw, ConstructType::Circle);
                construct_type_button(ui, aw, ConstructType::Text);
            });
        });
}

fn construct_type_button(ui: &mut Ui, ewa: &mut EventWriter<Act>, ct: ConstructType) {
    if ui.add(Button::new(format!("{ct}"))).clicked() {
        ewa.send(Act::Insert(ct));
    }
}
