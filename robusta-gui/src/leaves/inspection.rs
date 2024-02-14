use robusta_core::RobustaEntity;

pub fn view_inspection(
    ui: &mut egui::Ui,
    selected_entities: &mut Vec<(crate::uistate::SelectionInstance, RobustaEntity)>,
) {
    ui.colored_label(egui::Color32::WHITE, "Properties:");
    ui.separator();

    if selected_entities.is_empty() {
        ui.label("No Selected Entities.");
        return;
    }

    for entity in selected_entities {
        match &entity.1 {
            RobustaEntity::Point(a) => ui.label(format!("{a}")),
            RobustaEntity::Line(a) => ui.label(format!("{a}")),
            RobustaEntity::Arc(a) => ui.label(format!("{a}")),
            RobustaEntity::Circle(a) => ui.label(format!("{a}")),
            RobustaEntity::Text(a) => ui.label(format!("{a}")),
            RobustaEntity::None => continue,
        };

        ui.separator();
    }
}
