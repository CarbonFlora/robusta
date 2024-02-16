use bevy::prelude::{EventWriter, Vec2};
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
    // let a = selected_entities.retain(|x| x.1.is_some());
    for entity in selected_entities.iter().filter(|x| x.1.is_some()) {
        match &entity.1.as_ref().unwrap() {
            RobustaEntity::Point(a) => {
                let b = ui.selectable_label(false, format!("{a}")).clicked();
                if b {
                    let c = a.min_max();
                    act_write.send(Act::PullCameraFocus(bevy::prelude::Rect {
                        min: Vec2 { x: c[0], y: c[1] },
                        max: Vec2 { x: c[2], y: c[3] },
                    }));
                }
            } // RobustaEntity::Line(a) => ui.selectable_label(false, format!("{a}")),
            // RobustaEntity::Arc(a) => ui.selectable_label(false, format!("{a}")),
            // RobustaEntity::Circle(a) => ui.selectable_label(false, format!("{a}")),
            // RobustaEntity::Text(a) => ui.selectable_label(false, format!("{a}")),
            _ => (), //placeholder
        };

        ui.separator();
    }
}
