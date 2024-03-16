use super::*;

pub fn view_history(ui: &mut egui::Ui, history: &(Act, String)) {
    // ui.label(format!("Latest Action: {:?}", history.0));
    // ui.separator();
    ui.label(history.1.to_string());
}
