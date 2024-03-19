use egui::{Button, Ui};

use super::*;

pub fn update_snap_egui(
    aw: &mut EventWriter<Act>,
    ss: &mut ResMut<SnapSettings>,
    db: &mut (Option<SnapType>, String),
    context: &mut Query<&mut EguiContext, With<PrimaryWindow>>,
) {
    let w = match context.get_single() {
        Ok(w) => w,
        Err(_) => return,
    };

    egui::Window::new("Snap")
        .title_bar(false)
        .anchor(Align2::CENTER_BOTTOM, Vec2 { x: 0., y: 0. })
        .show(w.get(), |ui| {
            ui.label("Snap");
            ui.horizontal(|ui| {
                build_type_button(ui, aw, SnapType::Endpoint);
                build_type_button(ui, aw, SnapType::Midpoint);
                build_type_button(ui, aw, SnapType::Nthpoint(None));
                let nth_n = ui.add(TextEdit::singleline(&mut db.1).desired_width(15.0));
                build_type_button(ui, aw, SnapType::Intersection);
                build_type_button(ui, aw, SnapType::Perpendicular);
                build_type_button(ui, aw, SnapType::Tangent);

                if w.get().input(|x| x.key_pressed(egui::Key::N)) && db.1.is_empty() {
                    nth_n.request_focus();
                }
                if let Ok(a) = db.1.parse::<usize>() {
                    ss.nthpoint.1 = a;
                }
            });
        });
}

fn build_type_button(ui: &mut Ui, ewa: &mut EventWriter<Act>, st: SnapType) {
    if ui.add(Button::new(format!("{st}"))).clicked() {
        ewa.send(Act::ToggleSnap(st));
    }
}
