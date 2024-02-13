use robusta_core::RobustaEntity;

pub fn view_inspection(
    ui: &mut egui::Ui,
    selected_entities: &mut Vec<(crate::uistate::SelectionInstance, RobustaEntity)>,
) {
    let mut text = String::new();

    for entity in selected_entities {
        text += format!("Selected Entity: {:?}\n", entity.1).as_str();
    }

    if text.is_empty() {
        text += format!("No Selected Entities.").as_str();
    }

    ui.label(text);
}
