use self::keystroke::ModalResources;

use super::*;

pub struct CameraUIPlugin;
impl bevy::app::Plugin for CameraUIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraUIBuffer::new())
            .add_event::<Menu>()
            .add_systems(Update, update_cameraui);
    }
}

// #[derive(Debug, Default, Resource)]
// pub struct CADState {
//     pub cad_term: Option<String>,
//     pub insert_menu: Option<Option<ConstructType>>,
//     pub snap_menu: Option<Option<SnapType>>,
// }

// #[derive(Debug,  Event)]
#[derive(Debug, Default, Resource, Event, Clone)]
pub struct CameraUIBuffer {
    pub menu: Menu,
    pub nth: String,
}

impl CameraUIBuffer {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Default, Event, Clone, PartialEq)]
pub enum Menu {
    #[default]
    NoMenu,
    CadTerm(String),
    InsertMenu(Option<ConstructType>),
    SnapMenu(Option<SnapType>),
}

impl Menu {
    fn new() -> Self {
        Menu::default()
    }
}

fn update_cameraui(
    mut er: EventReader<Menu>,
    mut rmmr: ResMut<ModalResources>,
    mut rmcs: ResMut<CameraUIBuffer>,
) {
    for a in er.read() {
        rmcs.menu = a.clone();

        rmmr.mode = match a {
            Menu::NoMenu => Mode::Normal,
            Menu::CadTerm(_) => Mode::Typing,
            Menu::InsertMenu(_) => Mode::Insert,
            Menu::SnapMenu(_) => Mode::Snap,
        };
    }
}

impl std::fmt::Display for Menu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = match self {
            Menu::NoMenu => "Exited menu.",
            Menu::CadTerm(_) => "Opened CADTerm.",
            Menu::InsertMenu(_) => "Opened Insert menu.",
            Menu::SnapMenu(_) => "Opened Snap menu.",
        };
        f.write_fmt(format_args!("{}", a))
    }
}
