use bevy::prelude::KeyCode;

use crate::keystrokes::Actions;

pub fn view_pressed_keys(
    ui: &mut egui::Ui,
    pressed_keys: &[Option<KeyCode>; 2],
    actions: &Actions,
) {
    let mut text = String::new();
    for i in pressed_keys {
        match i {
            None => text += format!("No key is being pressed.\n").as_str(),
            Some(a) => text += format!("Pressed key: {:?}\n", a).as_str(),
        }
    }
    text += format!("\nAction Performed: {:?}\n", actions).as_str();
    ui.label(text);
}
