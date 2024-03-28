use bevy::{
    input::{keyboard::KeyboardInput, mouse::MouseButtonInput, ButtonState},
    prelude::*,
};

use super::*;

pub struct KeyStrokePlugin;
impl bevy::app::Plugin for KeyStrokePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ModeState::default())
            .add_systems(PreUpdate, capture_keystrokes)
            .add_systems(Update, update_mode);
    }
}

#[derive(Debug, Resource, Default, PartialEq, Clone)]
pub struct ModeState(pub Mode);

#[derive(Debug, Default, PartialEq, Clone)]
pub enum Mode {
    #[default]
    Normal,
    Typing,
    Insert,
    Snap,
}

fn update_mode(mut ewa: EventReader<Act>, mut rmkm: ResMut<ModeState>) {
    if let Some(Act::KeyState(sp)) = ewa.read().last() {
        rmkm.0 = sp.clone()
    }
}

pub fn capture_keystrokes(
    // ui_state: Res<UiState>,
    mr: Res<ModeState>,
    kb: EventReader<KeyboardInput>,
    mb: EventReader<MouseButtonInput>,
    mut act_write: EventWriter<Act>,
) {
    let buffer = to_buffer(kb, mb);

    let act = match mr.0 {
        Mode::Normal => normal_act(buffer),
        Mode::Typing => typing_act(buffer),
        Mode::Insert => insert_act(buffer),
        Mode::Snap => snap_act(buffer),
    };

    if act != Act::None {
        act_write.send(act);
    }
}

fn to_buffer(
    mut kb: EventReader<KeyboardInput>,
    mut mb: EventReader<MouseButtonInput>,
) -> [Option<KeyCode>; 2] {
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

    buffer
}

fn normal_act(buffer: [Option<KeyCode>; 2]) -> Act {
    match buffer {
        [None, Some(KeyCode::Escape)] => Act::Exit,
        [None, Some(KeyCode::KeyT)] => Act::CameraUIMenu(CameraUiMenu::CadTerm("".to_string())),
        [None, Some(KeyCode::KeyI)] => Act::CameraUIMenu(CameraUiMenu::InsertMenu(None)),
        [None, Some(KeyCode::KeyS)] => {
            Act::CameraUIMenu(CameraUiMenu::SnapMenu((None, "".to_string())))
        }
        [None, Some(KeyCode::ArrowLeft)] => Act::MoveCamera((-1., 0.)),
        [None, Some(KeyCode::ArrowDown)] => Act::MoveCamera((0., -1.)),
        [None, Some(KeyCode::ArrowUp)] => Act::MoveCamera((0., 1.)),
        [None, Some(KeyCode::ArrowRight)] => Act::MoveCamera((1., 0.)),
        [None, Some(KeyCode::KeyF)] => Act::FitView,
        [Some(KeyCode::ControlLeft), Some(KeyCode::KeyI)] => Act::ZoomCamera(-1.),
        [Some(KeyCode::ControlLeft), Some(KeyCode::KeyO)] => Act::ZoomCamera(1.),
        [_, Some(KeyCode::Insert)] => Act::Confirm,
        [None, Some(KeyCode::Semicolon)] | [Some(KeyCode::ShiftLeft), Some(KeyCode::Semicolon)] => {
            Act::CameraUIMenu(CameraUiMenu::CadTerm("".to_string()))
        }
        _ => Act::None,
    }
}

fn typing_act(buffer: [Option<KeyCode>; 2]) -> Act {
    match buffer {
        [None, Some(KeyCode::Escape)] => Act::Exit,
        [None, Some(KeyCode::Semicolon)] | [Some(KeyCode::ShiftLeft), Some(KeyCode::Semicolon)] => {
            Act::CameraUIMenu(CameraUiMenu::CadTerm("".to_string()))
        }
        _ => Act::None,
    }
}

fn insert_act(buffer: [Option<KeyCode>; 2]) -> Act {
    match buffer {
        [None, Some(KeyCode::Escape)] => Act::Exit,
        [None, Some(KeyCode::KeyP)] => Act::Insert(ConstructType::PointBy1Click),
        [None, Some(KeyCode::KeyL)] => Act::Insert(ConstructType::LineBy2Click),
        [None, Some(KeyCode::KeyA)] => Act::Insert(ConstructType::ArcByEndEndMid),
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

sort acts
#[derive(Event, Debug, Default, PartialEq, Clone)]
pub enum Act {
    //Unsorted
    #[default]
    None,
    Exit,
    QuitWithoutSaving,
    //Dock
    //CameraUI
    CameraUIMenu(CameraUiMenu),
    //RCamera
    MoveCamera((f32, f32)),
    ZoomCamera(f32),
    PullCameraFocus(Rect),
    FitView,
    //RSelection
    DeselectAll,
    Confirm,
    //Snap
    ToggleSnap(SnapType),
    ClearSnaps,
    //Keystroke
    KeyState(Mode),
    TryAct(String),
    //Phantom
    //Construction
    Insert(ConstructType),
    //Diagnostic
    //Tag
    ModifyTag(REntity, TagModify),
    Taglist(TagListModify),
    //Style
}
// builder = builder.add(UnsortedPlugin);
// builder = builder.add(DockPlugin);
// builder = builder.add(CameraUIPlugin);
// builder = builder.add(RCameraPlugin);
// builder = builder.add(RSelectionPlugin);
// builder = builder.add(SnapPlugin);
// builder = builder.add(KeyStrokePlugin);
// builder = builder.add(PhantomPlugin);
// builder = builder.add(ConstructionPlugin);
// builder = builder.add(DiagnosticPlugin);
// builder = builder.add(TagPlugin);
// builder = builder.add(StylePlugin);
