use bevy::{
    input::{keyboard::KeyboardInput, mouse::MouseButtonInput, ButtonState},
    prelude::*,
};

use super::*;

pub struct KeyStrokePlugin;
impl bevy::app::Plugin for KeyStrokePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ModalResources::new())
            .add_systems(PreUpdate, capture_keystrokes)
            .add_systems(Update, update_mode);
    }
}

#[derive(Debug, Resource, Default)]
pub struct ModalResources {
    pub mode: Mode,
}

impl ModalResources {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Default)]
pub enum Mode {
    #[default]
    Normal,
    Typing,
    Insert,
    Snap,
}

fn update_mode(mut er: EventReader<Menu>, mut rmmr: ResMut<ModalResources>) {
    for a in er.read() {
        rmmr.mode = match a {
            Menu::NoMenu => Mode::Normal,
            Menu::CadTerm(_) => Mode::Typing,
            Menu::InsertMenu(_) => Mode::Insert,
            Menu::SnapMenu(_) => Mode::Snap,
        };
    }
}

pub fn capture_keystrokes(
    // ui_state: Res<UiState>,
    mr: Res<ModalResources>,
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

    let act = match mr.mode {
        Mode::Normal => normal_act(buffer),
        Mode::Typing => typing_act(buffer),
        Mode::Insert => insert_act(buffer),
        Mode::Snap => snap_act(buffer),
    };

    if act != Act::None {
        act_write.send(act);
    }
}

fn normal_act(buffer: [Option<KeyCode>; 2]) -> Act {
    match buffer {
        [None, Some(KeyCode::Escape)] => Act::Exit,
        [None, Some(KeyCode::KeyT)] => Act::CameraUIMenu(Menu::CadTerm("".to_string())),
        [None, Some(KeyCode::KeyI)] => Act::CameraUIMenu(Menu::InsertMenu(None)),
        [None, Some(KeyCode::KeyS)] => Act::CameraUIMenu(Menu::SnapMenu((None, "".to_string()))),
        [None, Some(KeyCode::ArrowLeft)] => Act::MoveCamera((-1., 0.)),
        [None, Some(KeyCode::ArrowDown)] => Act::MoveCamera((0., -1.)),
        [None, Some(KeyCode::ArrowUp)] => Act::MoveCamera((0., 1.)),
        [None, Some(KeyCode::ArrowRight)] => Act::MoveCamera((1., 0.)),
        [None, Some(KeyCode::KeyF)] => Act::FitView,
        [Some(KeyCode::ControlLeft), Some(KeyCode::KeyI)] => Act::ZoomCamera(-1.),
        [Some(KeyCode::ControlLeft), Some(KeyCode::KeyO)] => Act::ZoomCamera(1.),
        [_, Some(KeyCode::Insert)] => Act::Confirm,
        [None, Some(KeyCode::Semicolon)] | [Some(KeyCode::ShiftLeft), Some(KeyCode::Semicolon)] => {
            Act::CameraUIMenu(Menu::CadTerm("".to_string()))
        }
        _ => Act::None,
    }
}

fn typing_act(buffer: [Option<KeyCode>; 2]) -> Act {
    match buffer {
        [None, Some(KeyCode::Escape)] => Act::Exit,
        [None, Some(KeyCode::Semicolon)] | [Some(KeyCode::ShiftLeft), Some(KeyCode::Semicolon)] => {
            Act::CameraUIMenu(Menu::CadTerm("".to_string()))
        }
        _ => Act::None,
    }
}

fn insert_act(buffer: [Option<KeyCode>; 2]) -> Act {
    match buffer {
        [None, Some(KeyCode::Escape)] => Act::Exit,
        [None, Some(KeyCode::KeyP)] => Act::Insert(ConstructType::PointBy1Click),
        [None, Some(KeyCode::KeyL)] => Act::Insert(ConstructType::LineBy2Click),
        [None, Some(KeyCode::KeyA)] => Act::Insert(ConstructType::Arc),
        [None, Some(KeyCode::KeyC)] => Act::Insert(ConstructType::Circle),
        [None, Some(KeyCode::KeyT)] => Act::Insert(ConstructType::Text),
        [_, Some(KeyCode::Insert)] => Act::Confirm,
        _ => Act::None,
    }
}

fn snap_act(buffer: [Option<KeyCode>; 2]) -> Act {
    match buffer {
        [None, Some(KeyCode::Escape)] => Act::Exit,
        [None, Some(KeyCode::KeyC)] => Act::ClearSnaps,
        [None, Some(KeyCode::KeyE)] => Act::ToggleSnap(SnapType::Endpoint),
        [None, Some(KeyCode::KeyM)] => Act::ToggleSnap(SnapType::Midpoint),
        [None, Some(KeyCode::KeyN)] => Act::ToggleSnap(SnapType::Nthpoint(None)),
        [None, Some(KeyCode::KeyI)] => Act::ToggleSnap(SnapType::Intersection),
        [None, Some(KeyCode::KeyP)] => Act::ToggleSnap(SnapType::Perpendicular),
        [None, Some(KeyCode::KeyT)] => Act::ToggleSnap(SnapType::Tangent),
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
    CameraUIMenu(Menu),
    TryAct(String),
    EguiFocus(EguiWindow),
    PullCameraFocus(Rect),
    FitView,
    MoveCamera((f32, f32)),
    ZoomCamera(f32),
    Insert(ConstructType),
    ToggleSnap(SnapType),
    ClearSnaps,
    ModifyTag(REntity, TagModify),
    ModifyTaglist(TagListModify),
}
