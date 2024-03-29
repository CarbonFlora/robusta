use bevy::prelude::*;

use crate::{Snaps, UiState};

pub fn capture_keystrokes(
    ui_state: Res<UiState>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
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

    for mousekey in mouse.get_pressed() {
        match mousekey {
            MouseButton::Left => buffer[1] = Some(KeyCode::Insert),
            MouseButton::Right => {
                buffer[0] = Some(KeyCode::AltLeft);
                buffer[1] = Some(KeyCode::Insert)
            }
            // MouseButton::Middle => todo!(),
            // MouseButton::Other(_) => todo!(),
            _ => (),
        }
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
        [None, Some(KeyCode::KeyH)] => Act::MoveCamera((-1., 0.)),
        [None, Some(KeyCode::KeyJ)] => Act::MoveCamera((0., -1.)),
        [None, Some(KeyCode::KeyK)] => Act::MoveCamera((0., 1.)),
        [None, Some(KeyCode::KeyL)] => Act::MoveCamera((1., 0.)),
        [None, Some(KeyCode::KeyI)] => Act::ZoomCamera(-1.),
        [None, Some(KeyCode::KeyO)] => Act::ZoomCamera(1.),
        [None, Some(KeyCode::Insert)] => Act::Confirm,
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

#[derive(Event, Debug, Default, PartialEq, Clone)]
pub enum Act {
    #[default]
    None,
    Exit,
    QuitWithoutSaving,
    DeselectAll,
    Confirm,
    OpenCADTerm,
    TryAct(String),
    NewPoint,
    NewLine,
    Inspect,
    PullCameraFocus(Rect),
    FitView,
    MoveCamera((f32, f32)),
    ZoomCamera(f32),
    ToggleSnap(Snaps),
    ToggleSnapOff,
}
