use bevy::prelude::{EventWriter, Rect};
use robusta_core::RobustaEntity;

use crate::keystrokes::Act;

pub fn view_inspection(
    ui: &mut egui::Ui,
    selected_entities: &Vec<(crate::uistate::SelectionInstance, Option<RobustaEntity>)>,
    act_write: &mut EventWriter<Act>,
) {
    // ui.colored_label(egui::Color32::WHITE, "Properties:");
    ui.separator();

    if selected_entities.is_empty() {
        ui.label("No Selected Entities.");
        return;
    }

    for entity in selected_entities.iter().filter(|x| x.1.is_some()) {
        match &entity.1.as_ref().unwrap() {
            RobustaEntity::Point(a) => {
                if ui.selectable_label(false, format!("{a}")).clicked() {
                    let c = a.min_max();
                    act_write.send(Act::PullCameraFocus(Rect::new(c.0, c.1, c.2, c.3)));
                }
            }
            RobustaEntity::Line(a) => {
                if ui.selectable_label(false, format!("{a}")).clicked() {
                    let c = a.min_max();
                    act_write.send(Act::PullCameraFocus(Rect::new(c.0, c.1, c.2, c.3)));
                }
            }
            RobustaEntity::Arc(a) => {
                if ui.selectable_label(false, format!("{a}")).clicked() {
                    let c = a.min_max();
                    act_write.send(Act::PullCameraFocus(Rect::new(c.0, c.1, c.2, c.3)));
                }
            }
            RobustaEntity::Circle(a) => {
                if ui.selectable_label(false, format!("{a}")).clicked() {
                    let c = a.min_max();
                    act_write.send(Act::PullCameraFocus(Rect::new(c.0, c.1, c.2, c.3)));
                }
            }
            RobustaEntity::Text(a) => {
                if ui.selectable_label(false, format!("{a}")).clicked() {
                    let c = a.min_max();
                    act_write.send(Act::PullCameraFocus(Rect::new(c.0, c.1, c.2, c.3)));
                }
            }
        };

        ui.separator();
    }
}
