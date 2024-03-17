use super::*;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct HistoryBuffer {
    pub latest_act: Act,
    pub all_history: String,
}

pub fn view_history(ui: &mut egui::Ui, hb: &HistoryBuffer) {
    // ui.label(format!("Latest Action: {:?}", history.0));
    // ui.separator();
    ui.label(hb.all_history.to_string());
}
