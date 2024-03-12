use super::*;

pub fn view_pressed_keys(ui: &mut egui::Ui, pressed_keys: &[Option<KeyCode>; 2], acts: Vec<&Act>) {
    let mut text = String::new();
    for i in pressed_keys {
        match i {
            None => text += "No key is being pressed.\n".to_string().as_str(),
            Some(a) => text += format!("Pressed key: {:?}\n", a).as_str(),
        }
    }
    text += format!("\nAction Performed: {:?}\n", acts).as_str();
    ui.label(text);
}
