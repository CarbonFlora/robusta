use crate::uistate::DoSomethingComplex;

pub fn view_inspection(ui: &mut egui::Ui, selected_entities: &mut Vec<DoSomethingComplex>) {
    let mut text = String::new();

    for entity in selected_entities {
        text += format!("Selected Entity: {:?}\n", entity).as_str();
    }
    // for entity in selected_entities.iter() {
    //     text += format!("Selected Entity: {:?}\n", entity).as_str();
    // }

    if text.is_empty() {
        text += format!("No Selected Entities.").as_str();
    }

    ui.label(text);
}
