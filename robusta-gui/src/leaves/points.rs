use bevy::utils::HashMap;
use robusta_dxf::wrapper::RFile;

pub fn view_points(ui: &mut egui::Ui, loaded_files: &HashMap<Option<String>, RFile>) {
    let mut text = String::new();
    for file in loaded_files {
        for point in file.1.iter_points() {
            text += format!("{}\n", point).as_str();
        }
    }
    ui.label(text);
}
