use egui::{Button, Ui};

use super::*;

pub fn update_snap_egui(
    ewa: &mut EventWriter<Act>,
    context: &mut Query<&mut EguiContext, With<PrimaryWindow>>,
    ss: &mut ResMut<SnapSettings>,
    db: &mut ResMut<DockBuffer>,
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
                build_type_button(ui, ewa, SnapType::Endpoint);
                build_type_button(ui, ewa, SnapType::Midpoint);
                build_type_button(ui, ewa, SnapType::Nthpoint(None));
                let nth_n = ui.add(TextEdit::singleline(&mut db.nth_n).desired_width(15.0));
                build_type_button(ui, ewa, SnapType::Intersection);
                build_type_button(ui, ewa, SnapType::Perpendicular);
                build_type_button(ui, ewa, SnapType::Tangent);

                if w.get().input(|x| x.key_pressed(egui::Key::N)) && db.nth_n.is_empty() {
                    nth_n.request_focus();
                }
                if let Ok(a) = db.nth_n.parse::<usize>() {
                    ss.nthpoint.1 = a;
                }
            });
        });
}

fn build_type_button(ui: &mut Ui, ewa: &mut EventWriter<Act>, st: SnapType) {
    if ui.add(Button::new(format!("{st}"))).clicked() {
        ewa.send(Act::ToggleSnap(Some(st)));
    }
}
