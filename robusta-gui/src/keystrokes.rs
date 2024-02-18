use bevy::prelude::*;

use crate::UiState;

pub fn capture_keystrokes(
    ui_state: Res<UiState>,
    keys: Res<Input<KeyCode>>,
    mut act_write: EventWriter<Act>,
) {
    let mut buffer = [None; 2];

    for keycode in keys.get_pressed() {
        match keycode {
            KeyCode::ControlLeft | KeyCode::ControlRight => buffer[0] = Some(KeyCode::ControlLeft),
            KeyCode::ShiftLeft | KeyCode::ShiftRight => buffer[0] = Some(KeyCode::ShiftLeft),
            KeyCode::AltLeft | KeyCode::AltRight => buffer[0] = Some(KeyCode::AltLeft),
            _ => buffer[1] = Some(*keycode),
        };
    }

    let act = match ui_state.cad_state.mode {
        crate::Mode::Normal => normal_act(buffer),
        crate::Mode::Typing => typing_act(buffer),
    };

    if act != Act::None {
        act_write.send(act);
    }
}

fn normal_act(buffer: [Option<KeyCode>; 2]) -> Act {
    match buffer {
        [None, Some(KeyCode::Escape)] => Act::Exit,
        [None, Some(KeyCode::H)] => Act::MoveCamera((-1., 0.)),
        [None, Some(KeyCode::J)] => Act::MoveCamera((0., -1.)),
        [None, Some(KeyCode::K)] => Act::MoveCamera((0., 1.)),
        [None, Some(KeyCode::L)] => Act::MoveCamera((1., 0.)),
        [None, Some(KeyCode::I)] => Act::ZoomCamera(-1.),
        [None, Some(KeyCode::O)] => Act::ZoomCamera(1.),
        [None, Some(KeyCode::Semicolon)] | [Some(KeyCode::ShiftLeft), Some(KeyCode::Semicolon)] => {
            Act::OpenCADTerm
        }
        _ => Act::None,
    }
}

fn typing_act(buffer: [Option<KeyCode>; 2]) -> Act {
    match buffer {
        [None, Some(KeyCode::Escape)] => Act::Exit,
        [None, Some(KeyCode::Semicolon)] | [Some(KeyCode::ShiftLeft), Some(KeyCode::Semicolon)] => {
            Act::OpenCADTerm
        }
        _ => Act::None,
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
    Inspect,
    PullCameraFocus(Rect),
    FitView,
    MoveCamera((f32, f32)),
    ZoomCamera(f32),
}
