use bevy::{
    input::{keyboard::KeyboardInput, mouse::MouseButtonInput, ButtonState},
    prelude::*,
};

use crate::{plugins::construction::ConstructType, SnapType, UiState};

pub struct KeyStrokePlugin;
impl bevy::app::Plugin for KeyStrokePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreUpdate, capture_keystrokes);
    }
}

pub fn capture_keystrokes(
    ui_state: Res<UiState>,
    mut kb: EventReader<KeyboardInput>,
    mut mb: EventReader<MouseButtonInput>,
    mut act_write: EventWriter<Act>,
) {
    let mut buffer = [None; 2];

    for k in kb.read().filter(|x| x.state == ButtonState::Pressed) {
        match k.key_code {
            KeyCode::ControlLeft | KeyCode::ControlRight => buffer[0] = Some(KeyCode::ControlLeft),
            KeyCode::ShiftLeft | KeyCode::ShiftRight => buffer[0] = Some(KeyCode::ShiftLeft),
            KeyCode::AltLeft | KeyCode::AltRight => buffer[0] = Some(KeyCode::AltLeft),
            _ => buffer[1] = Some(k.key_code),
        };
    }

    for m in mb.read().filter(|x| x.state == ButtonState::Pressed) {
        match m.button {
            MouseButton::Left => buffer[1] = Some(KeyCode::Insert),
            MouseButton::Right => {
                buffer[0] = Some(KeyCode::AltLeft);
                buffer[1] = Some(KeyCode::Insert)
            }
            _ => (),
        }
    }

    let act = match ui_state.cad_state.mode {
        crate::Mode::Normal => normal_act(buffer),
        crate::Mode::Typing => typing_act(buffer),
        crate::Mode::Insert => insert_act(buffer),
        crate::Mode::Snap => snap_act(buffer),
    };

    if act != Act::None {
        act_write.send(act);
    }
}

fn normal_act(buffer: [Option<KeyCode>; 2]) -> Act {
    match buffer {
        [None, Some(KeyCode::Escape)] => Act::Exit,
        [None, Some(KeyCode::KeyI)] => Act::Insert(None),
        [None, Some(KeyCode::KeyS)] => Act::ToggleSnap(None),
        [None, Some(KeyCode::ArrowLeft)] => Act::MoveCamera((-1., 0.)),
        [None, Some(KeyCode::ArrowDown)] => Act::MoveCamera((0., -1.)),
        [None, Some(KeyCode::ArrowUp)] => Act::MoveCamera((0., 1.)),
        [None, Some(KeyCode::ArrowRight)] => Act::MoveCamera((1., 0.)),
        [None, Some(KeyCode::KeyF)] => Act::FitView,
        [Some(KeyCode::ControlLeft), Some(KeyCode::KeyI)] => Act::ZoomCamera(-1.),
        [Some(KeyCode::ControlLeft), Some(KeyCode::KeyO)] => Act::ZoomCamera(1.),
        [_, Some(KeyCode::Insert)] => Act::Confirm,
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

fn insert_act(buffer: [Option<KeyCode>; 2]) -> Act {
    match buffer {
        [None, Some(KeyCode::Escape)] => Act::Exit,
        [None, Some(KeyCode::KeyP)] => Act::Insert(Some(ConstructType::Point)),
        [None, Some(KeyCode::KeyL)] => Act::Insert(Some(ConstructType::Line)),
        [None, Some(KeyCode::KeyA)] => Act::Insert(Some(ConstructType::Arc)),
        [None, Some(KeyCode::KeyC)] => Act::Insert(Some(ConstructType::Circle)),
        [None, Some(KeyCode::KeyT)] => Act::Insert(Some(ConstructType::Text)),
        _ => Act::None,
    }
}

fn snap_act(buffer: [Option<KeyCode>; 2]) -> Act {
    match buffer {
        [None, Some(KeyCode::Escape)] => Act::Exit,
        [None, Some(KeyCode::KeyC)] => Act::ToggleSnap(None),
        [None, Some(KeyCode::KeyE)] => Act::ToggleSnap(Some(SnapType::Endpoint)),
        [None, Some(KeyCode::KeyM)] => Act::ToggleSnap(Some(SnapType::Midpoint)),
        [None, Some(KeyCode::KeyN)] => Act::ToggleSnap(Some(SnapType::Nthpoint(None))),
        [None, Some(KeyCode::KeyI)] => Act::ToggleSnap(Some(SnapType::Intersection)),
        [None, Some(KeyCode::KeyP)] => Act::ToggleSnap(Some(SnapType::Perpendicular)),
        [None, Some(KeyCode::KeyT)] => Act::ToggleSnap(Some(SnapType::Tangent)),
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
    Inspect,
    Insert(Option<ConstructType>),
    PullCameraFocus(Rect),
    FitView,
    MoveCamera((f32, f32)),
    ZoomCamera(f32),
    ToggleSnap(Option<SnapType>),
}
