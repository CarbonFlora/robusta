use self::{
    keystroke::ModalResources,
    leaves::{insert::update_insert_egui, snap::update_snap_egui},
};

use super::*;

pub struct CameraUIPlugin;
impl bevy::app::Plugin for CameraUIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraUIBuffer::new())
            .add_event::<Menu>()
            .add_systems(Update, update_menu)
            .add_systems(Update, update_camera_ui);
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
    // pub nth: String,
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
    SnapMenu((Option<SnapType>, String)),
}

impl Menu {
    pub fn new() -> Self {
        Menu::default()
    }
}

#[allow(clippy::too_many_arguments)]
pub fn update_camera_ui(
    //Input
    mut rmcb: ResMut<CameraUIBuffer>,
    //Output
    mut aw: EventWriter<Act>,
    mut ewcui: EventWriter<Menu>,
    mut ss: ResMut<SnapSettings>,
    mut context: Query<&mut EguiContext, With<PrimaryWindow>>,
) {
    match &mut rmcb.menu {
        Menu::NoMenu => (),
        Menu::CadTerm(buffer) => update_terminal_egui(&mut aw, &mut ewcui, buffer, &mut context),
        Menu::InsertMenu(_buffer) => update_insert_egui(&mut aw, &mut context),
        Menu::SnapMenu(buffer) => update_snap_egui(&mut aw, &mut ss, buffer, &mut context),
    }
    // if uis.cad_state.cad_term.is_some() {
    //     update_terminal_egui(&mut aw, &mut uis, &mut ecp);
    // }
    // if uis.cad_state.insert_menu.is_some() {
    //     update_insert_egui(&mut aw, &mut ecp);
    // }
    // if uis.cad_state.snap_menu.is_some() {
    // update_snap_egui(&mut aw, &mut ecp, &mut ss, &mut db);
    // }
}

fn update_menu(mut er: EventReader<Menu>, mut rmcs: ResMut<CameraUIBuffer>) {
    for a in er.read() {
        rmcs.menu = a.clone();
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
