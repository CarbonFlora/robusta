use bevy::prelude::KeyCode;

pub fn view_pressed_keys(
    ui: &mut egui::Ui,
    pressed_keys: &[Option<KeyCode>; 2],
    // type_registry: &TypeRegistry,
    // world: &World,
    // selection: &mut InspectorSelection,
) {
    let mut text = String::new();
    for i in pressed_keys {
        match i {
            None => text += format!("No key is being pressed.\n").as_str(),
            Some(a) => text += format!("Pressed key: {:?}\n", a).as_str(),
        }
    }
    ui.label(text);
}
