use bevy::prelude::*;

pub fn capture_keystrokes(keys: Res<Input<KeyCode>>, mut act_write: EventWriter<Act>) {
    let mut buffer = [None; 2];

    for keycode in keys.get_pressed() {
        match keycode {
            KeyCode::ControlLeft | KeyCode::ControlRight => buffer[0] = Some(KeyCode::ControlLeft),
            KeyCode::ShiftLeft | KeyCode::ShiftRight => buffer[0] = Some(KeyCode::ShiftLeft),
            KeyCode::AltLeft | KeyCode::AltRight => buffer[0] = Some(KeyCode::AltLeft),
            _ => buffer[1] = Some(keycode.clone()),
        };
    }

    let act = match buffer {
        [None, Some(KeyCode::Escape)] => Act::Exit,
        [None, Some(KeyCode::Semicolon)] | [Some(KeyCode::ShiftLeft), Some(KeyCode::Semicolon)] => {
            Act::OpenCADTerm
        }
        _ => Act::None,
    };

    if act != Act::None {
        act_write.send(act);
    }
}

#[derive(Event, Debug, PartialEq)]
pub enum Act {
    None,
    Exit,
    QuitWithoutSaving,
    DeselectAll,
    OpenCADTerm,
    TryAct(String),
    NewPoint,
    DebugReMapSelection(Entity),
    DebugReMapSelectionsAll,
}

// impl Act {
//     fn to_act(&self, string: &String) -> Self {
//         return match string.as_str() {
//             "deselect" | "dsa" => Act::DeselectAll,
//             "point" | "p" => Act::NewPoint,
//             "q!" => Act::QuitWithoutSaving,
//             _ => Act::None,
//         };
//     }
// }
