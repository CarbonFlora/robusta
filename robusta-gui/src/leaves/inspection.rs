use bevy::prelude::{EventWriter, Rect};
// use robusta_core::RobustaEntity;

use crate::{keystrokes::Act, REntity};

pub fn view_inspection(
    ui: &mut egui::Ui,
    selected_entities: &Vec<REntity>,
    act_write: &mut EventWriter<Act>,
) {
    ui.separator();
    if selected_entities.is_empty() {
        ui.label("No Selected Entities.");
        return;
    }

    for re in selected_entities {
        match re {
            REntity::Arc(sp) => {
                if ui.selectable_label(false, format!("{sp}")).clicked() {
                    let c = sp.min_max();
                    act_write.send(Act::PullCameraFocus(Rect::new(c.0, c.1, c.2, c.3)));
                }
            }
            REntity::Circle(sp) => {
                if ui.selectable_label(false, format!("{sp}")).clicked() {
                    let c = sp.min_max();
                    act_write.send(Act::PullCameraFocus(Rect::new(c.0, c.1, c.2, c.3)));
                }
            }
            REntity::Line(sp) => {
                if ui.selectable_label(false, format!("{sp}")).clicked() {
                    let c = sp.min_max();
                    act_write.send(Act::PullCameraFocus(Rect::new(c.0, c.1, c.2, c.3)));
                }
            }
            REntity::Point(sp) => {
                if ui.selectable_label(false, format!("{sp}")).clicked() {
                    let c = sp.min_max();
                    act_write.send(Act::PullCameraFocus(Rect::new(c.0, c.1, c.2, c.3)));
                }
            }
            REntity::Text(sp) => {
                if ui.selectable_label(false, format!("{sp}")).clicked() {
                    let c = sp.min_max();
                    act_write.send(Act::PullCameraFocus(Rect::new(c.0, c.1, c.2, c.3)));
                }
            }
        }
    }
}
