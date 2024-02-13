use bevy::utils::HashMap;
use robusta_dxf::wrapper::RobustaEntities;

// use crate::uistate::*;
// use crate::*;

pub fn view_points(ui: &mut egui::Ui, loaded_files: &HashMap<Option<String>, RobustaEntities>) {
    let mut text = String::new();
    for file in loaded_files {
        for point in &file.1.points {
            text += format!("{}\n", point).as_str();
        }
    }
    ui.label(text);
}
